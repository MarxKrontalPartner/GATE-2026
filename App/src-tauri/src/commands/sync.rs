//! Tauri IPC boundary for field-level sync: no SQL or business rules here.
//!
//! Intended use from the app:
//! 1. `sync_take_pending` -> `{ endpoint, body }`
//! 2. PATCH `body` to `<API_BASE_URL>/<endpoint>` (200, 409 and 404 are expected answers)
//! 3. `sync_apply` with the response `status` and body, or status "FAILED"
//!    when no usable answer came back, plus the `sent` request body
//! 4. `sync_list_conflicts` lists stored conflicts (nothing changes them yet)
use sea_orm::TransactionTrait;
use serde_json::{json, Value};
use tauri::State;

use crate::{
    db::ClientDb,
    repositories::sync_repository as sync,
    services::domain_service::{self, DomainResource, SyncSpec},
};

fn synced_resource(resource: &str) -> Result<(DomainResource, SyncSpec), String> {
    let spec = domain_service::resource(resource)?;
    let sync = domain_service::sync_spec(spec.table)
        .ok_or_else(|| format!("{resource} is not synchronized"))?;
    Ok((spec, sync))
}

/// Merges the pending local edits into the request body and marks them as sending.
#[tauri::command]
pub async fn sync_take_pending(db: State<'_, ClientDb>, resource: String) -> Result<Value, String> {
    let (spec, sync_spec) = synced_resource(&resource)?;
    let txn = db.0.begin().await.map_err(|e| e.to_string())?;
    let body = sync::take_pending(&txn, spec.table).await?;
    txn.commit().await.map_err(|e| e.to_string())?;
    Ok(json!({ "endpoint": sync_spec.endpoint, "body": body }))
}

/// Stores the server answer: "OK" | "CONFLICT" | "NOT_FOUND", or "FAILED".
/// `sent` is the request body that was sent (from `sync_take_pending`).
#[tauri::command]
pub async fn sync_apply(db: State<'_, ClientDb>, resource: String, status: String, body: Value,
    sent: Vec<Value>) -> Result<(), String> {
    let (spec, _) = synced_resource(&resource)?;
    let txn = db.0.begin().await.map_err(|e| e.to_string())?;
    sync::apply_response(&txn, spec, &status, &body, &sent).await?;
    txn.commit().await.map_err(|e| e.to_string())
}

/// Records created offline as one bulk create: `{ endpoint, localIds, records }`.
/// `records` contain no local id; send them with POST to `endpoint`.
#[tauri::command]
pub async fn sync_take_creates(db: State<'_, ClientDb>, resource: String) -> Result<Value, String> {
    let (spec, sync_spec) = synced_resource(&resource)?;
    let creates = sync::take_creates(&db.0, spec).await?;
    let (local_ids, records): (Vec<i64>, Vec<Value>) = creates.into_iter().unzip();
    Ok(json!({ "endpoint": sync_spec.create_endpoint, "localIds": local_ids, "records": records }))
}

/// Stores the answer of the bulk create: status "OK" with the created server
/// records (same order as `local_ids`), or "FAILED" with an error text.
#[tauri::command]
pub async fn sync_apply_creates(db: State<'_, ClientDb>, resource: String, local_ids: Vec<i64>,
    status: String, created: Vec<Value>, error: Option<String>) -> Result<(), String> {
    let (spec, _) = synced_resource(&resource)?;
    sync::apply_creates(&db.0, spec, &local_ids, &status, &created, error).await
}

#[tauri::command]
pub async fn sync_list_conflicts(db: State<'_, ClientDb>) -> Result<Vec<Value>, String> {
    sync::list_open(&db.0).await
}
