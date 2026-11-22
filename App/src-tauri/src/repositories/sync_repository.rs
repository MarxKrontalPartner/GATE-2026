//! Field-level synchronization of offline edits with the Django `sync` action.
//!
//! Flow:
//! 1. `record_update` (called from `domain_repository::update`) writes one row
//!    per changed field into `client_sync_changes`:
//!    (table_name, column_name, row_pk) → server_value, new_value.
//!    Saving the same field again only updates `new_value`; changing it back
//!    to `server_value` removes the row.
//! 2. `take_pending` builds the request body from those rows:
//!    `[{"id": 2, "patch": [{"field", "base", "new"}]}]`.
//! 3. The app sends the body with PATCH to `SyncSpec::endpoint`.
//! 4. `apply_response` stores the answer: saved rows are written into the
//!    local table and their changes removed; conflicts are stored in
//!    `client_sync_conflicts` (and removed from the changes).
//!
//! The HTTP request itself is not made here; the app sends it with its
//! existing API helpers (auth headers, base URL).

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, Set,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::entities::{
    client_record_state as state, client_sync_changes as changes,
    client_sync_conflicts as conflicts, client_sync_outbox as outbox,
};
use sea_orm::{DatabaseConnection, TransactionTrait};
use crate::services::domain_service::{self, DomainResource};

fn now() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string()
}

fn err(e: impl ToString) -> String { e.to_string() }

type Patch = Vec<Value>;

/// Compares two values by meaning: numbers are equal when their values are
/// equal (`1` == `1.0`). JSON passed through the app's JavaScript loses the
/// difference between integers and decimals, so `==` alone is not enough.
pub fn same_value(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        (Value::Array(x), Value::Array(y)) =>
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| same_value(p, q)),
        (Value::Object(x), Value::Object(y)) =>
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| same_value(v, w))),
        _ => a == b,
    }
}

// ------------------------------------------------------------ record state

async fn get_state<C: ConnectionTrait>(c: &C, table: &str, id: i64)
    -> Result<Option<state::Model>, String> {
    state::Entity::find_by_id((table.to_string(), id.to_string()))
        .one(c).await.map_err(err)
}

/// Creates or updates the `client_record_state` row of a record.
/// `version`: new server version, `None` keeps the stored one.
async fn set_state<C: ConnectionTrait>(c: &C, table: &str, id: i64, status: &str,
    version: Option<i64>, bump_local: bool) -> Result<(), String> {
    match get_state(c, table, id).await? {
        Some(model) => {
            let local_revision = model.local_revision + i32::from(bump_local);
            let mut active = model.into_active_model();
            active.sync_status = Set(status.to_string());
            active.updated_at = Set(now());
            active.local_revision = Set(local_revision);
            if let Some(v) = version {
                active.base_server_revision = Set(Some(v.to_string()));
            }
            if status == "synced" {
                active.last_synced_at = Set(Some(now()));
            }
            active.update(c).await.map_err(err)?;
        }
        None => {
            state::ActiveModel {
                entity_table: Set(table.to_string()),
                local_pk: Set(id.to_string()),
                client_uuid: Set(Uuid::new_v4().to_string()),
                server_pk: Set(Some(id.to_string())),
                base_server_revision: Set(version.map(|v| v.to_string())),
                local_revision: Set(i32::from(bump_local)),
                sync_status: Set(status.to_string()),
                updated_at: Set(now()),
                last_synced_at: Set((status == "synced").then(now)),
                deleted_at: Set(None),
            }.insert(c).await.map_err(err)?;
        }
    }
    Ok(())
}

/// Stores the server version of a downloaded record.
pub async fn mark_synced<C: ConnectionTrait>(c: &C, table: &str, id: i64, version: Option<i64>)
    -> Result<(), String> {
    set_state(c, table, id, "synced", version, false).await
}

/// After the app saved a record directly on the server (online), the local
/// copy equals the server: its record state is `synced` with the new version.
/// The conflict table is left as it is.
pub async fn settle_after_server_write<C: ConnectionTrait>(c: &C, table: &str, id: i64,
    version: Option<i64>) -> Result<(), String> {
    set_state(c, table, id, "synced", version, false).await
}

// ----------------------------------------------------------------- changes

/// Unsent changes of one record, or of all records when `pk` is `None`.
async fn change_rows<C: ConnectionTrait>(c: &C, table: &str, pk: Option<&str>)
    -> Result<Vec<changes::Model>, String> {
    let mut query = changes::Entity::find().filter(changes::Column::TableName.eq(table));
    if let Some(pk) = pk {
        query = query.filter(changes::Column::RowPk.eq(pk));
    }
    query.order_by_asc(changes::Column::RowPk).all(c).await.map_err(err)
}

async fn find_change<C: ConnectionTrait>(c: &C, table: &str, column: &str, pk: &str)
    -> Result<Option<changes::Model>, String> {
    changes::Entity::find_by_id((table.to_string(), column.to_string(), pk.to_string()))
        .one(c).await.map_err(err)
}

async fn delete_change<C: ConnectionTrait>(c: &C, table: &str, column: &str, pk: &str)
    -> Result<(), String> {
    changes::Entity::delete_by_id((table.to_string(), column.to_string(), pk.to_string()))
        .exec(c).await.map_err(err)?;
    Ok(())
}

/// True when the record has changes that are not yet saved on the server.
pub(crate) async fn has_pending<C: ConnectionTrait>(c: &C, table: &str, pk: &str) -> Result<bool, String> {
    Ok(!change_rows(c, table, Some(pk)).await?.is_empty())
}

/// True when the server reported conflicts for the record that are still open.
async fn has_open_conflict<C: ConnectionTrait>(c: &C, table: &str, id: i64) -> Result<bool, String> {
    Ok(!open_conflicts(c, table, id).await?.is_empty())
}

/// Removes all unsent changes of a record (after an online save put the
/// current values on the server).
pub(crate) async fn clear_changes<C: ConnectionTrait>(c: &C, table: &str, id: i64) -> Result<(), String> {
    let pk = id.to_string();
    for row in change_rows(c, table, Some(&pk)).await? {
        delete_change(c, table, &row.column_name, &pk).await?;
    }
    Ok(())
}

/// Why a download must not overwrite the local record, if it must not:
/// "changes" = unsent offline changes, "conflict" = the last sync reported a conflict.
pub(crate) async fn keep_local_reason<C: ConnectionTrait>(c: &C, table: &str, id: i64)
    -> Result<Option<&'static str>, String> {
    if has_pending(c, table, &id.to_string()).await? {
        return Ok(Some("changes"));
    }
    let conflict = get_state(c, table, id).await?.is_some_and(|s| s.sync_status == "conflict");
    Ok(conflict.then_some("conflict"))
}

/// Builds the patch of a single local save. Only fields in `fields` are synced.
pub fn diff(fields: &[&str], before: &Value, after: &Value) -> Patch {
    fields.iter().filter_map(|field| {
        let base = before.get(*field).cloned().unwrap_or(Value::Null);
        let new = after.get(*field).cloned().unwrap_or(Value::Null);
        (!same_value(&base, &new)).then(|| json!({ "field": field, "base": base, "new": new }))
    }).collect()
}

/// Writes one changed field: insert, update `new_value`, or remove the row
/// when the field is back at its server value.
async fn upsert_change<C: ConnectionTrait>(c: &C, table: &str, pk: &str, field: &str,
    base: &Value, new: &Value) -> Result<(), String> {
    match find_change(c, table, field, pk).await? {
        Some(row) if same_value(&row.server_value, new) => delete_change(c, table, field, pk).await?,
        Some(row) => {
            let mut active = row.into_active_model();
            active.new_value = Set(new.clone());
            active.update(c).await.map_err(err)?;
        }
        None => {
            changes::ActiveModel {
                table_name: Set(table.to_string()),
                column_name: Set(field.to_string()),
                row_pk: Set(pk.to_string()),
                server_value: Set(base.clone()),
                new_value: Set(new.clone()),
            }.insert(c).await.map_err(err)?;
        }
    }
    Ok(())
}

/// Called inside the transaction of `domain_repository::update` (offline saves).
pub async fn record_update<C: ConnectionTrait>(c: &C, table: &str, id: i64, fields: &[&str],
    before: &Value, after: &Value) -> Result<(), String> {
    // A record created offline is not on the server yet: its create sends the
    // latest values, so its edits need no change rows.
    if get_state(c, table, id).await?.is_some_and(|s| s.sync_status == "new") {
        return Ok(());
    }
    let patch = diff(fields, before, after);
    let pk = id.to_string();
    for op in &patch {
        upsert_change(c, table, &pk, op["field"].as_str().unwrap_or_default(), &op["base"], &op["new"]).await?;
    }
    // Fields with an open conflict: the user saw the server value in the form,
    // so it becomes the base of the change. The other fields are unaffected.
    let rebased = rebase_conflicts(c, table, id, after).await?;
    if patch.is_empty() && !rebased {
        return Ok(());
    }
    let status = if has_pending(c, table, &pk).await? { "pending" } else { "synced" };
    set_state(c, table, id, status, None, !patch.is_empty()).await
}

/// Sets both values of a change row (insert or update); removes the row when
/// the new value equals the base.
async fn set_change<C: ConnectionTrait>(c: &C, table: &str, pk: &str, field: &str,
    base: &Value, new: &Value) -> Result<(), String> {
    if same_value(base, new) {
        if find_change(c, table, field, pk).await?.is_some() {
            delete_change(c, table, field, pk).await?;
        }
        return Ok(());
    }
    match find_change(c, table, field, pk).await? {
        Some(row) => {
            let mut active = row.into_active_model();
            active.server_value = Set(base.clone());
            active.new_value = Set(new.clone());
            active.update(c).await.map_err(err)?;
        }
        None => {
            changes::ActiveModel {
                table_name: Set(table.to_string()),
                column_name: Set(field.to_string()),
                row_pk: Set(pk.to_string()),
                server_value: Set(base.clone()),
                new_value: Set(new.clone()),
            }.insert(c).await.map_err(err)?;
        }
    }
    Ok(())
}

/// Open conflicts of a record, one per field.
async fn open_conflicts<C: ConnectionTrait>(c: &C, table: &str, id: i64)
    -> Result<Vec<conflicts::Model>, String> {
    conflicts::Entity::find()
        .filter(conflicts::Column::EntityTable.eq(table))
        .filter(conflicts::Column::LocalPk.eq(id.to_string()))
        .filter(conflicts::Column::Status.eq("unresolved"))
        .all(c).await.map_err(err)
}

async fn mark_conflict<C: ConnectionTrait>(c: &C, row: conflicts::Model, status: &str) -> Result<(), String> {
    let mut active = row.into_active_model();
    active.status = Set(status.to_string());
    active.update(c).await.map_err(err)?;
    Ok(())
}

/// Saving the form locally after a conflict: each conflicting field takes the
/// server value as base and the saved value as new value (no change row when
/// they are equal). The conflict is then handled (`resolved_local`).
/// Returns true when the record had open conflicts.
async fn rebase_conflicts<C: ConnectionTrait>(c: &C, table: &str, id: i64, after: &Value)
    -> Result<bool, String> {
    let open = open_conflicts(c, table, id).await?;
    if open.is_empty() {
        return Ok(false);
    }
    let pk = id.to_string();
    for row in open {
        let new = after.get(&row.column_name).cloned().unwrap_or(Value::Null);
        set_change(c, table, &pk, &row.column_name, &row.server_value, &new).await?;
        mark_conflict(c, row, "resolved_local").await?;
    }
    Ok(true)
}

/// An online save put the form's values on the server: the record's open
/// conflicts are handled.
pub(crate) async fn resolve_conflicts_by_online_save<C: ConnectionTrait>(c: &C, table: &str, id: i64)
    -> Result<(), String> {
    for row in open_conflicts(c, table, id).await? {
        mark_conflict(c, row, "resolved_local").await?;
    }
    Ok(())
}

/// Builds the request body `[{"id": 2, "patch": [{"field", "base", "new"}]}]`
/// from all unsent changes. Fields keep the order of the sync field list.
pub async fn take_pending<C: ConnectionTrait>(c: &C, table: &str) -> Result<Vec<Value>, String> {
    let order = domain_service::sync_spec(table).map(|s| s.fields).unwrap_or(&[]);
    let position = |field: &str| order.iter().position(|f| *f == field).unwrap_or(usize::MAX);
    let mut groups: Vec<(i64, Vec<changes::Model>)> = Vec::new();
    for row in change_rows(c, table, None).await? {
        let id: i64 = row.row_pk.parse().map_err(err)?;
        match groups.iter_mut().find(|(g, _)| *g == id) {
            Some((_, rows)) => rows.push(row),
            None => groups.push((id, vec![row])),
        }
    }
    groups.sort_by_key(|(id, _)| *id);
    Ok(groups.into_iter().map(|(id, mut rows)| {
        rows.sort_by_key(|r| position(&r.column_name));
        let patch: Patch = rows.into_iter().map(|r| json!({
            "field": r.column_name, "base": r.server_value, "new": r.new_value,
        })).collect();
        json!({ "id": id, "patch": patch })
    }).collect())
}

// ---------------------------------------------------------------- response

/// The patch that was sent for record `id`.
fn sent_patch(sent: &[Value], id: i64) -> Patch {
    sent.iter().find(|item| item["id"].as_i64() == Some(id))
        .and_then(|item| item["patch"].as_array()).cloned().unwrap_or_default()
}

/// The server now holds the values that were sent for `id`. Changes that are
/// still the sent value are removed; a field changed again meanwhile stays,
/// based on the value the server now has.
async fn clear_sent<C: ConnectionTrait>(c: &C, table: &str, id: i64, patch: &Patch) -> Result<(), String> {
    let pk = id.to_string();
    for op in patch {
        let field = op["field"].as_str().unwrap_or_default();
        if let Some(row) = find_change(c, table, field, &pk).await? {
            if same_value(&row.new_value, &op["new"]) {
                delete_change(c, table, field, &pk).await?;
            } else {
                let mut active = row.into_active_model();
                active.server_value = Set(op["new"].clone());
                active.update(c).await.map_err(err)?;
            }
        }
    }
    Ok(())
}

/// Stores the server answer to the request `sent`.
/// `status`: "OK" | "CONFLICT" | "NOT_FOUND" from the response body, or
/// "FAILED" when the request did not get a usable answer.
pub async fn apply_response<C: ConnectionTrait>(c: &C, spec: DomainResource, status: &str,
    body: &Value, sent: &[Value]) -> Result<(), String> {
    let table = spec.table;
    let sent_ids: Vec<i64> = sent.iter().filter_map(|item| item["id"].as_i64()).collect();

    match status {
        "OK" | "CONFLICT" => {}
        "NOT_FOUND" => {
            // The server saved nothing. Records it does not know can never be
            // synced: their changes are removed. The others are sent again later.
            for id in body["ids"].as_array().into_iter().flatten().filter_map(Value::as_i64) {
                let pk = id.to_string();
                for row in change_rows(c, table, Some(&pk)).await? {
                    delete_change(c, table, &row.column_name, &pk).await?;
                }
                set_state(c, table, id, "failed", None, false).await?;
            }
            return Ok(());
        }
        // Network or server error: the changes stay and are sent next time.
        _ => return Ok(()),
    }

    let mut handled: Vec<i64> = Vec::new();

    for row in body["saved"].as_array().into_iter().flatten() {
        let id = row["id"].as_i64().ok_or("Saved row without id")?;
        clear_sent(c, table, id, &sent_patch(sent, id)).await?;
        let pending = has_pending(c, table, &id.to_string()).await?;
        // A record with an open conflict keeps the local values of the conflicting fields.
        let conflict = has_open_conflict(c, table, id).await?;
        if !pending && !conflict {
            crate::repositories::domain_repository::apply_server_row(c, spec, id, row).await?;
        }
        let status = if pending { "pending" } else if conflict { "conflict" } else { "synced" };
        set_state(c, table, id, status, row["version"].as_i64(), false).await?;
        handled.push(id);
    }

    for item in body["conflicts"].as_array().into_iter().flatten() {
        let id = item["id"].as_i64().ok_or("Conflict without id")?;
        let pk = id.to_string();
        let patch = sent_patch(sent, id);
        let server_version = item["version"].as_i64().map(|v| v.to_string());
        // One conflict row per conflicting field (updated, not duplicated, when the
        // same conflict is reported again). All changes of the record stay in
        // client_sync_changes and are sent again with the next sync.
        for conflict in item["conflicts"].as_array().into_iter().flatten() {
            let field = conflict["field"].as_str().ok_or("Conflict without field")?;
            let new_value = patch.iter().find(|op| op["field"] == field)
                .map(|op| op["new"].clone()).unwrap_or(Value::Null);
            let existing = conflicts::Entity::find()
                .filter(conflicts::Column::EntityTable.eq(table))
                .filter(conflicts::Column::LocalPk.eq(pk.as_str()))
                .filter(conflicts::Column::ColumnName.eq(field))
                .filter(conflicts::Column::Status.eq("unresolved"))
                .one(c).await.map_err(err)?;
            match existing {
                Some(row) => {
                    let mut active = row.into_active_model();
                    active.server_value = Set(conflict["server_value"].clone());
                    active.new_value = Set(new_value);
                    active.server_revision = Set(server_version.clone());
                    active.update(c).await.map_err(err)?;
                }
                None => {
                    conflicts::ActiveModel {
                        conflict_id: Set(Uuid::new_v4().to_string()),
                        entity_table: Set(table.to_string()),
                        local_pk: Set(pk.clone()),
                        column_name: Set(field.to_string()),
                        server_value: Set(conflict["server_value"].clone()),
                        new_value: Set(new_value),
                        server_revision: Set(server_version.clone()),
                        status: Set("unresolved".to_string()),
                        created_at: Set(now()),
                    }.insert(c).await.map_err(err)?;
                }
            }
        }
        set_state(c, table, id, "conflict", None, false).await?;
        handled.push(id);
    }

    // Neither saved nor conflicting: the server already had these values.
    for id in sent_ids.into_iter().filter(|id| !handled.contains(id)) {
        clear_sent(c, table, id, &sent_patch(sent, id)).await?;
        if !has_pending(c, table, &id.to_string()).await? {
            let status = if has_open_conflict(c, table, id).await? { "conflict" } else { "synced" };
            set_state(c, table, id, status, None, false).await?;
        }
    }
    Ok(())
}

// ------------------------------------------------- records created offline

/// Fields not sent when creating a record: the local id (the server assigns
/// its own), the server version and media links (files are not synced).
const NOT_SENT_ON_CREATE: &[&str] = &["id", "version", "bilder", "messprotokolle", "titelBild"];

/// Called inside the transaction of `domain_repository::create` (offline):
/// status "new" in client_record_state and a "create" row in client_sync_outbox.
pub async fn record_create<C: ConnectionTrait>(c: &C, table: &str, temp_id: i64) -> Result<(), String> {
    let uuid = Uuid::new_v4().to_string();
    state::ActiveModel {
        entity_table: Set(table.to_string()),
        local_pk: Set(temp_id.to_string()),
        client_uuid: Set(uuid.clone()),
        server_pk: Set(None),
        base_server_revision: Set(None),
        local_revision: Set(0),
        sync_status: Set("new".to_string()),
        updated_at: Set(now()),
        last_synced_at: Set(None),
        deleted_at: Set(None),
    }.insert(c).await.map_err(err)?;
    outbox::ActiveModel {
        operation_id: Set(uuid),
        entity_table: Set(table.to_string()),
        local_pk: Set(temp_id.to_string()),
        operation: Set("create".to_string()),
        base_server_revision: Set(None),
        payload_json: Set(None),
        status: Set("pending".to_string()),
        attempt_count: Set(0),
        last_error: Set(None),
        created_at: Set(now()),
        last_attempt_at: Set(None),
        acknowledged_at: Set(None),
    }.insert(c).await.map_err(err)?;
    Ok(())
}

async fn create_rows<C: ConnectionTrait>(c: &C, table: &str, statuses: &[&str])
    -> Result<Vec<outbox::Model>, String> {
    outbox::Entity::find()
        .filter(outbox::Column::EntityTable.eq(table))
        .filter(outbox::Column::Operation.eq("create"))
        .filter(outbox::Column::Status.is_in(statuses.iter().copied()))
        .order_by_asc(outbox::Column::CreatedAt)
        .all(c).await.map_err(err)
}

/// Records created offline, in creation order, as (local id, record without
/// local id) for one bulk create. Marks their outbox rows "sending"; rows left
/// "sending" by an interrupted sync are sent again.
pub async fn take_creates(db: &DatabaseConnection, spec: DomainResource) -> Result<Vec<(i64, Value)>, String> {
    let txn = db.begin().await.map_err(err)?;
    let mut ids = Vec::new();
    for row in create_rows(&txn, spec.table, &["pending", "sending"]).await? {
        ids.push(row.local_pk.parse::<i64>().map_err(err)?);
        let attempts = row.attempt_count + 1;
        let mut active = row.into_active_model();
        active.status = Set("sending".to_string());
        active.attempt_count = Set(attempts);
        active.last_attempt_at = Set(Some(now()));
        active.update(&txn).await.map_err(err)?;
    }
    txn.commit().await.map_err(err)?;
    let mut creates = Vec::new();
    for id in ids {
        let mut record = crate::repositories::domain_repository::get(db, spec, id).await?;
        if let Some(fields) = record.as_object_mut() {
            for field in NOT_SENT_ON_CREATE { fields.remove(*field); }
        }
        creates.push((id, record));
    }
    Ok(creates)
}

/// Stores the answer to the bulk create. `status` "OK": `created` are the
/// server records in the order of `local_ids`; each record gets its server id,
/// the server values and the status "synced". Otherwise the creates are sent
/// again next time (`error` is kept in the outbox row).
pub async fn apply_creates(db: &DatabaseConnection, spec: DomainResource, local_ids: &[i64],
    status: &str, created: &[Value], error: Option<String>) -> Result<(), String> {
    let table = spec.table;
    let txn = db.begin().await.map_err(err)?;
    if status == "OK" && created.len() == local_ids.len() {
        for (local, row) in local_ids.iter().zip(created) {
            let server_id = row["id"].as_i64().ok_or("Created record without id")?;
            crate::repositories::domain_repository::remap_id(&txn, spec, *local, server_id).await?;
            crate::repositories::domain_repository::apply_server_row(&txn, spec, server_id, row).await?;
            if let Some(model) = get_state(&txn, table, server_id).await? {
                let mut active = model.into_active_model();
                active.server_pk = Set(Some(server_id.to_string()));
                active.sync_status = Set("synced".to_string());
                active.base_server_revision = Set(row["version"].as_i64().map(|v| v.to_string()));
                active.updated_at = Set(now());
                active.last_synced_at = Set(Some(now()));
                active.update(&txn).await.map_err(err)?;
            }
            for op in create_rows(&txn, table, &["sending"]).await?.into_iter()
                .filter(|op| op.local_pk == server_id.to_string()) {
                let mut active = op.into_active_model();
                active.status = Set("acknowledged".to_string());
                active.acknowledged_at = Set(Some(now()));
                active.update(&txn).await.map_err(err)?;
            }
        }
    } else {
        let message = error.unwrap_or_else(|| "Create request failed".to_string());
        for op in create_rows(&txn, table, &["sending"]).await? {
            let mut active = op.into_active_model();
            active.status = Set("pending".to_string());
            active.last_error = Set(Some(message.clone()));
            active.update(&txn).await.map_err(err)?;
        }
    }
    txn.commit().await.map_err(err)
}

// --------------------------------------------------------------- conflicts

/// Unresolved conflicts, one entry per field, newest first
/// (read only; nothing changes them yet).
pub async fn list_open<C: ConnectionTrait>(c: &C) -> Result<Vec<Value>, String> {
    Ok(conflicts::Entity::find()
        .filter(conflicts::Column::Status.eq("unresolved"))
        .order_by_desc(conflicts::Column::CreatedAt)
        .all(c).await.map_err(err)?
        .into_iter()
        .map(|m| json!({
            "conflictId": m.conflict_id,
            "table": m.entity_table,
            "id": m.local_pk.parse::<i64>().ok(),
            "field": m.column_name,
            "serverValue": m.server_value,
            "newValue": m.new_value,
            "serverVersion": m.server_revision.and_then(|v| v.parse::<i64>().ok()),
            "createdAt": m.created_at,
        }))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::domain_repository as repo;
    use sea_orm::DatabaseConnection;

    const TABLE: &str = "bauwerksdiagnostik_rueckprallhammer";

    async fn test_db() -> DatabaseConnection {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("SQLite in-memory pool");
        sqlx::migrate!("./migrations").run(&pool).await.expect("migrations");
        sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool)
    }

    fn spec() -> DomainResource { domain_service::resource("Rueckprallhammer").unwrap() }

    /// Imports record 5 as downloaded from the server with version 3.
    async fn seed(db: &DatabaseConnection) {
        repo::import(db, spec(), vec![json!({
            "id": 5, "version": 3, "datum": "2026-10-31", "bemerkung": "Messstelle 1",
            "sort_order": 1, "hammertyp": "N", "schlagrichtung": "horizontal",
            "abgeschlossen": false, "oberflaecheGeschliffen": null,
        })]).await.expect("import");
    }

    /// Saves record 5 like the app's forms do: the whole record with some fields changed.
    async fn edit(db: &DatabaseConnection, changes: Value) {
        let mut row = repo::get(db, spec(), 5).await.expect("load record");
        for (key, value) in changes.as_object().unwrap() {
            row[key] = value.clone();
        }
        repo::update(db, spec(), 5, row).await.expect("local save");
    }

    /// (column_name, server_value, new_value) of all change rows.
    async fn rows(db: &DatabaseConnection) -> Vec<(String, Value, Value)> {
        changes::Entity::find().order_by_asc(changes::Column::ColumnName).all(db).await.unwrap()
            .into_iter().map(|r| (r.column_name, r.server_value, r.new_value)).collect()
    }

    async fn record_state(db: &DatabaseConnection) -> state::Model {
        get_state(db, TABLE, 5).await.unwrap().expect("record state")
    }

    fn server_row(bemerkung: &str, version: i64) -> Value {
        json!({"id": 5, "version": version, "datum": "2026-10-31", "bemerkung": bemerkung,
            "sort_order": 1, "hammertyp": "N", "schlagrichtung": "horizontal",
            "abgeschlossen": false, "bilder": [], "bearbeiter": [], "messprotokolle": []})
    }

    #[test]
    fn numbers_compare_by_value() {
        assert!(same_value(&json!(1), &json!(1.0)));
        assert!(same_value(&json!([1, 2.0]), &json!([1.0, 2])));
        assert!(!same_value(&json!(1), &json!(1.5)));
        assert!(!same_value(&json!(1), &json!("1")));
    }

    #[test]
    fn sync_clears_change_when_javascript_turned_decimal_into_integer() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"medianRueckprallwert": 40.0})).await;
            // The app sends the request through JavaScript, where 40.0 becomes 40.
            let sent = vec![json!({"id": 5, "patch": [
                {"field": "medianRueckprallwert", "base": null, "new": 40}]})];
            apply_response(&db, spec(), "OK", &json!({"status": "OK", "conflicts": [],
                "saved": [{"id": 5, "version": 4, "medianRueckprallwert": 40.0}]}), &sent).await.unwrap();
            assert!(rows(&db).await.is_empty());
            assert_eq!(record_state(&db).await.sync_status, "synced");
        });
    }

    async fn create_offline(db: &DatabaseConnection, bemerkung: &str) -> Value {
        repo::create(db, spec(), json!({
            "datum": "2026-11-22", "bemerkung": bemerkung, "hammertyp": "N",
            "schlagrichtung": "horizontal", "sort_order": 1,
        })).await.expect("offline create")
    }

    async fn outbox_rows(db: &DatabaseConnection) -> Vec<(String, String, String)> {
        outbox::Entity::find().order_by_asc(outbox::Column::CreatedAt).all(db).await.unwrap()
            .into_iter().map(|r| (r.local_pk, r.operation, r.status)).collect()
    }

    #[test]
    fn offline_create_gets_temporary_id_and_status_new() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            let first = create_offline(&db, "erste").await;
            let second = create_offline(&db, "zweite").await;
            assert_eq!(first["id"], json!(-1));
            assert_eq!(second["id"], json!(-2));
            let s = get_state(&db, TABLE, -1).await.unwrap().unwrap();
            assert_eq!(s.sync_status, "new");
            assert_eq!(s.server_pk, None);
            assert_eq!(outbox_rows(&db).await, vec![
                ("-1".into(), "create".into(), "pending".into()),
                ("-2".into(), "create".into(), "pending".into()),
            ]);
        });
    }

    #[test]
    fn editing_a_new_record_writes_no_change_rows() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            let created = create_offline(&db, "neu").await;
            let mut row = created.clone();
            row["bemerkung"] = json!("geändert");
            repo::update(&db, spec(), -1, row).await.unwrap();
            assert!(rows(&db).await.is_empty());
            assert_eq!(get_state(&db, TABLE, -1).await.unwrap().unwrap().sync_status, "new");
        });
    }

    #[test]
    fn creates_are_sent_without_local_id_in_creation_order() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            create_offline(&db, "erste").await;
            create_offline(&db, "zweite").await;
            let creates = take_creates(&db, spec()).await.unwrap();
            assert_eq!(creates.iter().map(|(id, _)| *id).collect::<Vec<_>>(), vec![-1, -2]);
            assert_eq!(creates[0].1["bemerkung"], json!("erste"));
            assert!(creates.iter().all(|(_, r)| r.get("id").is_none() && r.get("version").is_none()));
            assert!(outbox_rows(&db).await.iter().all(|(_, _, status)| status == "sending"));
        });
    }

    #[test]
    fn server_ids_replace_temporary_ids_and_status_becomes_synced() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            create_offline(&db, "erste").await;
            create_offline(&db, "zweite").await;
            let creates = take_creates(&db, spec()).await.unwrap();
            let ids: Vec<i64> = creates.iter().map(|(id, _)| *id).collect();
            let created = vec![
                json!({"id": 41, "version": 1, "datum": "2026-11-22", "bemerkung": "erste",
                    "hammertyp": "N", "schlagrichtung": "horizontal", "sort_order": 1}),
                json!({"id": 42, "version": 1, "datum": "2026-11-22", "bemerkung": "zweite",
                    "hammertyp": "N", "schlagrichtung": "horizontal", "sort_order": 1}),
            ];
            apply_creates(&db, spec(), &ids, "OK", &created, None).await.unwrap();
            assert!(repo::get(&db, spec(), -1).await.is_err());
            assert_eq!(repo::get(&db, spec(), 42).await.unwrap()["bemerkung"], json!("zweite"));
            let s = get_state(&db, TABLE, 41).await.unwrap().unwrap();
            assert_eq!(s.sync_status, "synced");
            assert_eq!(s.server_pk.as_deref(), Some("41"));
            assert_eq!(s.base_server_revision.as_deref(), Some("1"));
            assert_eq!(outbox_rows(&db).await, vec![
                ("41".into(), "create".into(), "acknowledged".into()),
                ("42".into(), "create".into(), "acknowledged".into()),
            ]);
            assert!(take_creates(&db, spec()).await.unwrap().is_empty());
        });
    }

    #[test]
    fn failed_create_is_sent_again() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            create_offline(&db, "erste").await;
            let ids: Vec<i64> = take_creates(&db, spec()).await.unwrap().into_iter().map(|(id, _)| id).collect();
            apply_creates(&db, spec(), &ids, "FAILED", &[], Some("HTTP 400".into())).await.unwrap();
            assert_eq!(outbox_rows(&db).await, vec![("-1".into(), "create".into(), "pending".into())]);
            assert_eq!(get_state(&db, TABLE, -1).await.unwrap().unwrap().sync_status, "new");
            assert_eq!(take_creates(&db, spec()).await.unwrap().len(), 1);
        });
    }

    #[test]
    fn other_forms_sync_the_same_way() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            let riss = domain_service::resource("Rissbreitenmessung").unwrap();
            let table = "bauwerksdiagnostik_rissbreitenmessung";
            repo::import(&db, riss, vec![json!({
                "id": 3, "version": 2, "datum": "2026-10-31", "bemerkung": "", "sort_order": 1,
                "messmittel": "risslupe", "maxRissbreite": 0.2, "risse": [{"bezeichnung": "R1", "breite": 0.2}],
            })]).await.unwrap();
            // Offline edit of a number and the JSON list of cracks.
            let mut row = repo::get(&db, riss, 3).await.unwrap();
            row["maxRissbreite"] = json!(0.4);
            row["risse"] = json!([{"bezeichnung": "R1", "breite": 0.4}]);
            repo::update(&db, riss, 3, row).await.unwrap();
            assert_eq!(take_pending(&db, table).await.unwrap(), vec![json!({"id": 3, "patch": [
                {"field": "risse", "base": [{"bezeichnung": "R1", "breite": 0.2}],
                 "new": [{"bezeichnung": "R1", "breite": 0.4}]},
                {"field": "maxRissbreite", "base": 0.2, "new": 0.4},
            ]})]);
            // Offline create gets a temporary id and status "new".
            let created = repo::create(&db, riss, json!({"datum": "2026-11-22", "messmittel": "risslupe"}))
                .await.unwrap();
            assert_eq!(created["id"], json!(-1));
            assert_eq!(get_state(&db, table, -1).await.unwrap().unwrap().sync_status, "new");
            assert_eq!(take_creates(&db, riss).await.unwrap().len(), 1);
        });
    }

    #[test]
    fn diff_only_includes_synced_fields() {
        let patch = diff(&["hammertyp", "bemerkung"],
            &json!({"hammertyp": "N", "bemerkung": "x", "sort_order": 1}),
            &json!({"hammertyp": "L", "bemerkung": "x", "sort_order": 2}));
        assert_eq!(patch, vec![json!({"field": "hammertyp", "base": "N", "new": "L"})]);
    }

    #[test]
    fn import_stores_server_version() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            let s = record_state(&db).await;
            assert_eq!(s.base_server_revision.as_deref(), Some("3"));
            assert_eq!(s.sync_status, "synced");
        });
    }

    #[test]
    fn offline_save_stores_one_row_per_changed_field() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"bemerkung": "Test 1", "abgeschlossen": true})).await;
            let saved = changes::Entity::find().all(&db).await.unwrap();
            assert_eq!(saved.len(), 2);
            assert!(saved.iter().all(|r| r.table_name == TABLE && r.row_pk == "5"));
            assert_eq!(rows(&db).await, vec![
                ("abgeschlossen".into(), json!(false), json!(true)),
                ("bemerkung".into(), json!("Messstelle 1"), json!("Test 1")),
            ]);
            assert_eq!(record_state(&db).await.sync_status, "pending");
        });
    }

    #[test]
    fn saving_a_field_again_keeps_server_value_and_updates_new_value() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"hammertyp": "L"})).await;
            edit(&db, json!({"hammertyp": "pendel"})).await;
            assert_eq!(rows(&db).await, vec![("hammertyp".into(), json!("N"), json!("pendel"))]);
        });
    }

    #[test]
    fn changing_a_field_back_removes_the_change() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"bemerkung": "neu"})).await;
            edit(&db, json!({"bemerkung": "Messstelle 1"})).await;
            assert!(rows(&db).await.is_empty());
            assert_eq!(record_state(&db).await.sync_status, "synced");
            assert!(take_pending(&db, TABLE).await.unwrap().is_empty());
        });
    }

    #[test]
    fn request_has_the_api_format() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"datum": "2026-11-01", "oberflaecheGeschliffen": true})).await;
            assert_eq!(take_pending(&db, TABLE).await.unwrap(), vec![json!({"id": 5, "patch": [
                {"field": "oberflaecheGeschliffen", "base": null, "new": true},
                {"field": "datum", "base": "2026-10-31", "new": "2026-11-01"},
            ]})]);
        });
    }

    #[test]
    fn non_synced_table_records_nothing() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            let projekt = domain_service::resource("Projekt").unwrap();
            let created = repo::create(&db, projekt, json!({
                "name": "P", "projektnummer": "P-1", "bezeichnung": "P"
            })).await.unwrap();
            repo::update(&db, projekt, created["id"].as_i64().unwrap(),
                json!({"name": "Q"})).await.unwrap();
            assert!(rows(&db).await.is_empty());
        });
    }

    #[test]
    fn ok_response_writes_server_row_and_removes_changes() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"abgeschlossen": true})).await;
            let sent = take_pending(&db, TABLE).await.unwrap();
            apply_response(&db, spec(), "OK", &json!({"status": "OK", "conflicts": [], "saved": [{
                "id": 5, "version": 4, "abgeschlossen": true, "bemerkung": "vom Server",
            }]}), &sent).await.unwrap();
            assert!(rows(&db).await.is_empty());
            let s = record_state(&db).await;
            assert_eq!(s.sync_status, "synced");
            assert_eq!(s.base_server_revision.as_deref(), Some("4"));
            assert_eq!(repo::get(&db, spec(), 5).await.unwrap()["bemerkung"], json!("vom Server"));
        });
    }

    #[test]
    fn field_changed_while_sending_stays_based_on_sent_value() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"bemerkung": "erste"})).await;
            let sent = take_pending(&db, TABLE).await.unwrap();
            edit(&db, json!({"bemerkung": "zweite"})).await;
            apply_response(&db, spec(), "OK", &json!({"status": "OK", "conflicts": [],
                "saved": [server_row("erste", 4)]}), &sent).await.unwrap();
            assert_eq!(rows(&db).await, vec![("bemerkung".into(), json!("erste"), json!("zweite"))]);
            assert_eq!(repo::get(&db, spec(), 5).await.unwrap()["bemerkung"], json!("zweite"));
            assert_eq!(record_state(&db).await.sync_status, "pending");
        });
    }

    #[test]
    fn conflict_keeps_all_changes_of_the_record() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"hammertyp": "pendel", "abgeschlossen": true})).await;
            let sent = take_pending(&db, TABLE).await.unwrap();
            apply_response(&db, spec(), "CONFLICT", &json!({"status": "CONFLICT", "saved": [],
                "conflicts": [{"id": 5, "version": 6,
                    "conflicts": [{"field": "hammertyp", "server_value": "L"}]}]}), &sent).await.unwrap();
            let open = list_open(&db).await.unwrap();
            assert_eq!(open.len(), 1);
            assert_eq!(open[0]["field"], json!("hammertyp"));
            assert_eq!(open[0]["serverValue"], json!("L"));
            assert_eq!(open[0]["newValue"], json!("pendel"));
            assert_eq!(open[0]["serverVersion"], json!(6));
            // Every change of the record stays, the conflicting one included.
            assert_eq!(rows(&db).await, vec![
                ("abgeschlossen".into(), json!(false), json!(true)),
                ("hammertyp".into(), json!("N"), json!("pendel")),
            ]);
            assert_eq!(record_state(&db).await.sync_status, "conflict");
        });
    }

    /// Offline: Hammertyp N → pendel and Abgeschlossen → true; the server has Hammertyp L.
    async fn conflict_on_hammertyp(db: &DatabaseConnection) {
        seed(db).await;
        edit(db, json!({"hammertyp": "pendel", "abgeschlossen": true})).await;
        let sent = take_pending(db, TABLE).await.unwrap();
        apply_response(db, spec(), "CONFLICT", &json!({"status": "CONFLICT", "saved": [],
            "conflicts": [{"id": 5, "version": 6,
                "conflicts": [{"field": "hammertyp", "server_value": "L"}]}]}), &sent).await.unwrap();
    }

    #[test]
    fn saving_after_conflict_uses_server_value_as_base() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            conflict_on_hammertyp(&db).await;
            // The user saves the form again (here: also changes Bemerkung).
            edit(&db, json!({"bemerkung": "geprüft"})).await;
            assert_eq!(rows(&db).await, vec![
                ("abgeschlossen".into(), json!(false), json!(true)),
                ("bemerkung".into(), json!("Messstelle 1"), json!("geprüft")),
                ("hammertyp".into(), json!("L"), json!("pendel")),
            ]);
            assert!(list_open(&db).await.unwrap().is_empty());
            assert_eq!(record_state(&db).await.sync_status, "pending");
            assert_eq!(take_pending(&db, TABLE).await.unwrap(), vec![json!({"id": 5, "patch": [
                {"field": "hammertyp", "base": "L", "new": "pendel"},
                {"field": "bemerkung", "base": "Messstelle 1", "new": "geprüft"},
                {"field": "abgeschlossen", "base": false, "new": true},
            ]})]);
        });
    }

    #[test]
    fn saving_the_server_value_removes_the_conflicting_change() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            conflict_on_hammertyp(&db).await;
            edit(&db, json!({"hammertyp": "L"})).await;
            assert_eq!(rows(&db).await, vec![("abgeschlossen".into(), json!(false), json!(true))]);
            assert!(list_open(&db).await.unwrap().is_empty());
        });
    }

    #[test]
    fn online_save_handles_open_conflicts() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            conflict_on_hammertyp(&db).await;
            repo::mirror(&db, spec(), vec![server_row("online", 7)], true).await.unwrap();
            assert!(list_open(&db).await.unwrap().is_empty());
            assert!(rows(&db).await.is_empty());
            assert_eq!(record_state(&db).await.sync_status, "synced");
        });
    }

    #[test]
    fn conflicting_record_is_sent_again_without_duplicate_conflicts() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"hammertyp": "pendel", "abgeschlossen": true})).await;
            let sent = take_pending(&db, TABLE).await.unwrap();
            let conflict = json!({"status": "CONFLICT", "saved": [],
                "conflicts": [{"id": 5, "version": 6,
                    "conflicts": [{"field": "hammertyp", "server_value": "L"}]}]});
            apply_response(&db, spec(), "CONFLICT", &conflict, &sent).await.unwrap();
            // The next sync sends the same changes again.
            let again = take_pending(&db, TABLE).await.unwrap();
            assert_eq!(again, sent);
            // The same conflict reported again is stored once.
            apply_response(&db, spec(), "CONFLICT", &conflict, &again).await.unwrap();
            assert_eq!(list_open(&db).await.unwrap().len(), 1);
            assert_eq!(rows(&db).await.len(), 2);
            // Downloads do not overwrite the record.
            let result = repo::mirror(&db, spec(), vec![server_row("server", 7)], false).await.unwrap();
            assert_eq!(result.skipped, vec![json!({"id": 5, "reason": "changes"})]);
        });
    }

    #[test]
    fn not_found_removes_changes_of_unknown_record() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"abgeschlossen": true})).await;
            let sent = take_pending(&db, TABLE).await.unwrap();
            apply_response(&db, spec(), "NOT_FOUND", &json!({"status": "NOT_FOUND", "ids": [5]}), &sent)
                .await.unwrap();
            assert!(rows(&db).await.is_empty());
            assert_eq!(record_state(&db).await.sync_status, "failed");
        });
    }

    #[test]
    fn failed_request_keeps_changes() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"abgeschlossen": true})).await;
            let sent = take_pending(&db, TABLE).await.unwrap();
            apply_response(&db, spec(), "FAILED", &json!({}), &sent).await.unwrap();
            assert_eq!(rows(&db).await.len(), 1);
            assert_eq!(take_pending(&db, TABLE).await.unwrap(), sent);
        });
    }

    #[test]
    fn mirror_updates_local_copy_without_changes() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            repo::mirror(&db, spec(), vec![server_row("online gespeichert", 4)], false).await.unwrap();
            assert_eq!(repo::get(&db, spec(), 5).await.unwrap()["bemerkung"], json!("online gespeichert"));
            assert!(rows(&db).await.is_empty());
            assert_eq!(record_state(&db).await.base_server_revision.as_deref(), Some("4"));
        });
    }

    #[test]
    fn online_save_overwrites_record_with_unsent_changes() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"bemerkung": "offline"})).await;
            let result = repo::mirror(&db, spec(), vec![server_row("online gespeichert", 4)], true).await.unwrap();
            assert_eq!(result.written, 1);
            assert_eq!(repo::get(&db, spec(), 5).await.unwrap()["bemerkung"], json!("online gespeichert"));
            assert!(rows(&db).await.is_empty());
            assert_eq!(record_state(&db).await.sync_status, "synced");
        });
    }

    #[test]
    fn mirror_creates_record_created_online() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            assert_eq!(repo::mirror(&db, spec(), vec![server_row("neu", 1)], false).await.unwrap().written, 1);
            assert_eq!(repo::get(&db, spec(), 5).await.unwrap()["bemerkung"], json!("neu"));
        });
    }

    #[test]
    fn mirror_keeps_unsent_local_edits() {
        tauri::async_runtime::block_on(async {
            let db = test_db().await;
            seed(&db).await;
            edit(&db, json!({"bemerkung": "offline"})).await;
            let result = repo::mirror(&db, spec(), vec![server_row("server", 4)], false).await.unwrap();
            assert_eq!(result.written, 0);
            assert_eq!(result.skipped, vec![json!({"id": 5, "reason": "changes"})]);
            assert_eq!(repo::get(&db, spec(), 5).await.unwrap()["bemerkung"], json!("offline"));
            assert_eq!(rows(&db).await.len(), 1);
        });
    }
}
