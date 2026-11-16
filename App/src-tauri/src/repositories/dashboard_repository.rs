//! Dashboard projections through SeaORM entity finders.
//! This repository never modifies local domain records or performs synchronization.
use std::collections::{HashMap, HashSet};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, Set, TransactionTrait,
};
use serde::Serialize;
use serde_json::{json, Map, Value};
use crate::entities::{
    assetverwaltung_bauwerk as bauwerk,
    assetverwaltung_bauwerk_projekt as bauwerk_projekt,
    client_record_state as record_state,
    client_sync_conflicts as conflicts,
    client_dashboard_bauwerk_baseline as baseline,
    client_dashboard_available_bauwerk as available,
    client_dashboard_recent_bauwerk as recent,
    client_dashboard_meta as meta,
};

const BAUWERK_TABLE: &str = "assetverwaltung_bauwerk";
fn timestamp() -> String { chrono::Utc::now().to_rfc3339() }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardBauwerk {
    pub id: i64,
    pub bezeichnung: String,
    pub bauwerksnummer: String,
    pub archived: bool,
    pub projekt_ids: Vec<i64>,
    pub sync_status: String,
    pub server_update_available: bool,
    pub has_baseline: bool,
    pub last_opened_at: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableBauwerk {
    pub id: i64,
    pub bezeichnung: String,
    pub bauwerksnummer: String,
    pub archived: bool,
    pub projekt_ids: Vec<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSnapshot {
    pub bauwerke: Vec<DashboardBauwerk>,
    pub available: Vec<AvailableBauwerk>,
    pub last_server_check: Option<String>,
}

/// Compare only Bauwerk and project association fields supported by the current
/// Django response. Investigation revision tracking needs a future API contract.
fn server_snapshot(row: &Value) -> Result<String, String> {
    let record = row.as_object().ok_or("Bauwerk must be an object")?;
    let mut fields = Map::new();
    for name in ["bezeichnung", "archived", "beschreibung", "strasse", "bauweise",
                 "bauwerksnummer", "bauwerkstyp", "bemerkung"] {
        fields.insert(name.into(), record.get(name).cloned().unwrap_or(Value::Null));
    }
    let mut projects: Vec<i64> = record.get("projekt")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_i64).collect())
        .unwrap_or_default();
    projects.sort_unstable();
    projects.dedup();
    fields.insert("projekt".into(), json!(projects));
    serde_json::to_string(&fields).map_err(|e| e.to_string())
}

fn local_snapshot(item: &bauwerk::Model, projects: &[i64]) -> Result<String, String> {
    server_snapshot(&json!({
        "bezeichnung": item.bezeichnung, "archived": item.archived,
        "beschreibung": item.beschreibung, "strasse": item.strasse,
        "bauweise": item.bauweise, "bauwerksnummer": item.bauwerksnummer,
        "bauwerkstyp": item.bauwerkstyp, "bemerkung": item.bemerkung,
        "projekt": projects,
    }))
}

pub async fn snapshot(db: &DatabaseConnection) -> Result<DashboardSnapshot, String> {
    // Load complete entities: adding a field never requires adjusting SELECT lists.
    let items = bauwerk::Entity::find().all(db).await.map_err(|e| e.to_string())?;
    let links = bauwerk_projekt::Entity::find().all(db).await.map_err(|e| e.to_string())?;
    let states = record_state::Entity::find()
        .filter(record_state::Column::EntityTable.eq(BAUWERK_TABLE))
        .all(db).await.map_err(|e| e.to_string())?;
    let unresolved = conflicts::Entity::find()
        .filter(conflicts::Column::EntityTable.eq(BAUWERK_TABLE))
        .filter(conflicts::Column::Status.eq("unresolved"))
        .all(db).await.map_err(|e| e.to_string())?;
    let baselines = baseline::Entity::find().all(db).await.map_err(|e| e.to_string())?;
    let last_opened = recent::Entity::find().all(db).await.map_err(|e| e.to_string())?;
    let remote = available::Entity::find().all(db).await.map_err(|e| e.to_string())?;
    let last_server_check = meta::Entity::find_by_id("last_server_check".to_string())
        .one(db).await.map_err(|e| e.to_string())?.map(|item| item.value);

    let mut projects: HashMap<i64, Vec<i64>> = HashMap::new();
    for link in links { projects.entry(link.bauwerk_id).or_default().push(link.projekt_id); }
    for ids in projects.values_mut() { ids.sort_unstable(); ids.dedup(); }
    let state_map: HashMap<String, record_state::Model> = states.into_iter()
        .map(|s| (s.local_pk.clone(), s)).collect();
    let conflict_ids: HashSet<String> = unresolved.into_iter().map(|c| c.local_pk).collect();
    let baseline_map: HashMap<i64, baseline::Model> = baselines.into_iter()
        .map(|b| (b.bauwerk_id, b)).collect();
    let recent_map: HashMap<i64, String> = last_opened.into_iter()
        .map(|b| (b.bauwerk_id, b.last_opened_at)).collect();

    let mut bauwerke = Vec::new();
    for item in items {
        let id = item.id;
        let saved = baseline_map.get(&id);
        let local_diff = saved.map(|b| local_snapshot(&item, projects.get(&id).map(Vec::as_slice).unwrap_or(&[]))
            .map(|current| current != b.server_snapshot_json)).transpose()?.unwrap_or(false);
        let pending = state_map.get(&id.to_string()).is_some_and(|s| s.sync_status != "synced");
        let conflict = conflict_ids.contains(&id.to_string());
        let sync_status = if conflict { "conflict" }
            else if saved.is_none() || local_diff || pending { "unsynced" }
            else { "synced" };
        bauwerke.push(DashboardBauwerk {
            id, bezeichnung: item.bezeichnung, bauwerksnummer: item.bauwerksnummer,
            archived: item.archived, projekt_ids: projects.remove(&id).unwrap_or_default(),
            sync_status: sync_status.to_owned(),
            server_update_available: saved.is_some_and(|b| b.server_update_available),
            has_baseline: saved.is_some(),
            last_opened_at: recent_map.get(&id).cloned(),
        });
    }
    bauwerke.sort_by(|a, b| b.last_opened_at.cmp(&a.last_opened_at)
        .then_with(|| a.bezeichnung.to_lowercase().cmp(&b.bezeichnung.to_lowercase())));
    let mut available_rows = remote.into_iter().map(|r| AvailableBauwerk {
        id: r.bauwerk_id, bezeichnung: r.bezeichnung,
        bauwerksnummer: r.bauwerksnummer.unwrap_or_default(),
        archived: r.archived,
        projekt_ids: serde_json::from_str(&r.projekte_json).unwrap_or_default(),
    }).collect::<Vec<_>>();
    available_rows.sort_by(|a, b| a.bezeichnung.to_lowercase().cmp(&b.bezeichnung.to_lowercase()));
    Ok(DashboardSnapshot { bauwerke, available: available_rows, last_server_check })
}

/// Store a baseline only if the downloaded server data equals the local copy.
/// This prevents declaring an already locally edited Bauwerk "synced".
pub async fn record_baselines(db: &DatabaseConnection, incoming: Vec<Value>) -> Result<usize, String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    let mut recorded = 0;
    for obj in incoming {
        let id = obj["id"].as_i64().ok_or("Bauwerk missing id")?;
        if baseline::Entity::find_by_id(id).one(&txn).await.map_err(|e| e.to_string())?.is_some() { continue; }
        let Some(local) = bauwerk::Entity::find_by_id(id).one(&txn).await.map_err(|e| e.to_string())? else { continue; };
        let links = bauwerk_projekt::Entity::find()
            .filter(bauwerk_projekt::Column::BauwerkId.eq(id))
            .all(&txn).await.map_err(|e| e.to_string())?;
        let ids = links.iter().map(|l| l.projekt_id).collect::<Vec<_>>();
        if local_snapshot(&local, &ids)? != server_snapshot(&obj)? { continue; }
        // An existing record explicitly marked as pending must not be reclassified.
        if record_state::Entity::find_by_id((BAUWERK_TABLE.to_string(), id.to_string()))
            .one(&txn).await.map_err(|e| e.to_string())?
            .is_some_and(|s| s.sync_status != "synced") { continue; }
        baseline::ActiveModel {
            bauwerk_id: Set(id), server_snapshot_json: Set(server_snapshot(&obj)?),
            downloaded_at: Set(timestamp()), last_checked_at: Set(None),
            server_update_available: Set(false), seen_at: Set(Some(timestamp())),
        }.insert(&txn).await.map_err(|e| e.to_string())?;
        recorded += 1;
    }
    txn.commit().await.map_err(|e| e.to_string())?;
    Ok(recorded)
}

/// Compare server catalog to the last downloaded baseline; no domain writes.
pub async fn record_server_check(db: &DatabaseConnection, incoming: Vec<Value>) -> Result<(), String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    let local = bauwerk::Entity::find().all(&txn).await.map_err(|e| e.to_string())?;
    let local_ids: HashSet<i64> = local.into_iter().map(|r| r.id).collect();
    let baselines = baseline::Entity::find().all(&txn).await.map_err(|e| e.to_string())?;
    let baseline_map: HashMap<i64, baseline::Model> = baselines.into_iter()
        .map(|b| (b.bauwerk_id, b)).collect();
    // Rebuild server-only catalog, removing entries no longer returned by Django.
    available::Entity::delete_many().exec(&txn).await.map_err(|e| e.to_string())?;
    for obj in incoming {
        let id = obj["id"].as_i64().ok_or("Bauwerk missing id")?;
        if let Some(previous) = baseline_map.get(&id) {
            let mut update = previous.clone().into_active_model();
            update.server_update_available = Set(previous.server_snapshot_json != server_snapshot(&obj)?);
            update.last_checked_at = Set(Some(timestamp()));
            update.update(&txn).await.map_err(|e| e.to_string())?;
        } else if !local_ids.contains(&id) {
            let projects = obj.get("projekt").cloned().unwrap_or(json!([]));
            available::ActiveModel {
                bauwerk_id: Set(id),
                bezeichnung: Set(obj["bezeichnung"].as_str().unwrap_or("Ohne Bezeichnung").to_string()),
                bauwerksnummer: Set(obj["bauwerksnummer"].as_str().map(str::to_string)),
                archived: Set(obj["archived"].as_bool().unwrap_or(false)),
                projekte_json: Set(projects.to_string()),
                last_seen_at: Set(timestamp()),
            }.insert(&txn).await.map_err(|e| e.to_string())?;
        }
    }
    let now = timestamp();
    match meta::Entity::find_by_id("last_server_check".to_string())
        .one(&txn).await.map_err(|e| e.to_string())? {
        Some(existing) => {
            let mut update = existing.into_active_model();
            update.value = Set(now);
            update.update(&txn).await.map_err(|e| e.to_string())?;
        }
        None => {
            meta::ActiveModel { key: Set("last_server_check".to_string()), value: Set(now) }
                .insert(&txn).await.map_err(|e| e.to_string())?;
        }
    }
    txn.commit().await.map_err(|e| e.to_string())
}

pub async fn record_opened(db: &DatabaseConnection, id: i64) -> Result<(), String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    if bauwerk::Entity::find_by_id(id).one(&txn).await.map_err(|e| e.to_string())?.is_none() {
        return Err(format!("Local Bauwerk {id} not found"));
    }
    match recent::Entity::find_by_id(id).one(&txn).await.map_err(|e| e.to_string())? {
        Some(existing) => {
            let mut update = existing.into_active_model();
            update.last_opened_at = Set(timestamp());
            update.update(&txn).await.map_err(|e| e.to_string())?;
        }
        None => {
            recent::ActiveModel { bauwerk_id: Set(id), last_opened_at: Set(timestamp()) }
                .insert(&txn).await.map_err(|e| e.to_string())?;
        }
    }
    txn.commit().await.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use sea_orm::SqlxSqliteConnector;

    fn remote(id: i64, name: &str) -> Value {
        json!({
            "id": id, "bezeichnung": name, "archived": false,
            "beschreibung": "", "strasse": "", "bauweise": "Stahlbeton",
            "bauwerksnummer": "DEMO-01", "bauwerkstyp": null, "bemerkung": null,
            "projekt": [],
        })
    }

    #[test]
    fn snapshot_canonicalizes_project_order() {
        let mut original = remote(1, "Demo");
        original["projekt"] = json!([4, 1]);
        let mut reordered = original.clone();
        reordered["projekt"] = json!([1, 4]);
        assert_eq!(server_snapshot(&original).unwrap(), server_snapshot(&reordered).unwrap());
    }

    #[test]
    fn read_only_server_checks_preserve_local_bauwerke() {
        tauri::async_runtime::block_on(async {
            let pool = SqlitePoolOptions::new().max_connections(1)
                .connect_with(SqliteConnectOptions::new().in_memory(true))
                .await.expect("test sqlite");
            sqlx::migrate!("./migrations").run(&pool).await.expect("migrations");
            let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
            bauwerk::ActiveModel {
                id: Set(1), bezeichnung: Set("Demo".into()), archived: Set(false),
                beschreibung: Set("".into()), strasse: Set("".into()),
                bauweise: Set("Stahlbeton".into()), bauwerksnummer: Set("DEMO-01".into()),
                bauwerkstyp: Set(None), bemerkung: Set(None),
            }.insert(&db).await.expect("local Bauwerk");

            assert_eq!(record_baselines(&db, vec![remote(1, "Demo")]).await.unwrap(), 1);
            let first = snapshot(&db).await.unwrap();
            assert_eq!(first.bauwerke[0].sync_status, "synced");

            // Django now advertises a newer version plus another Bauwerk.
            record_server_check(&db, vec![remote(1, "Demo changed"), remote(2, "New remote")])
                .await.expect("comparison check");
            let second = snapshot(&db).await.unwrap();
            assert!(second.bauwerke[0].server_update_available);
            assert_eq!(second.available.len(), 1);
            assert_eq!(second.available[0].id, 2);
            let unchanged = bauwerk::Entity::find_by_id(1).one(&db).await.unwrap().unwrap();
            assert_eq!(unchanged.bezeichnung, "Demo");

            // Local edits remain distinguishable even when an update is available.
            let mut edited = unchanged.into_active_model();
            edited.bezeichnung = Set("Local edit".into());
            edited.update(&db).await.unwrap();
            let third = snapshot(&db).await.unwrap();
            assert_eq!(third.bauwerke[0].sync_status, "unsynced");
            assert!(third.bauwerke[0].server_update_available);

            record_opened(&db, 1).await.unwrap();
            assert!(snapshot(&db).await.unwrap().bauwerke[0].last_opened_at.is_some());
        });
    }
}
