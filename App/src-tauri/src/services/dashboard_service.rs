//! Dashboard metadata and status calculation, not the synchronization engine.
use std::collections::HashSet;
use sea_orm::DatabaseConnection;
use serde_json::Value;
use crate::repositories::dashboard_repository as repo;

fn validate_server_catalog(rows: &[Value]) -> Result<(), String> {
    let mut ids = HashSet::new();
    for item in rows {
        let id = item.get("id").and_then(Value::as_i64)
            .ok_or("Server Bauwerk is missing a numeric id")?;
        if id <= 0 || !ids.insert(id) { return Err(format!("Invalid or duplicate Bauwerk id {id}")); }
        if item.get("bezeichnung").and_then(Value::as_str).is_none() {
            return Err(format!("Server Bauwerk {id} is missing bezeichnung"));
        }
    }
    Ok(())
}

pub async fn snapshot(db: &DatabaseConnection) -> Result<repo::DashboardSnapshot, String> {
    repo::snapshot(db).await
}
pub async fn record_downloaded(db: &DatabaseConnection, rows: Vec<Value>) -> Result<usize, String> {
    validate_server_catalog(&rows)?;
    repo::record_baselines(db, rows).await
}
pub async fn record_server_check(db: &DatabaseConnection, rows: Vec<Value>) -> Result<(), String> {
    validate_server_catalog(&rows)?;
    repo::record_server_check(db, rows).await
}
pub async fn record_opened(db: &DatabaseConnection, id: i64) -> Result<(), String> {
    if id <= 0 { return Err("Invalid Bauwerk id".into()); }
    repo::record_opened(db, id).await
}
