//! Small IPC surface: metadata reads/checks. No upload or sync commands.
use serde_json::Value;
use tauri::State;
use crate::{db::ClientDb, repositories::dashboard_repository as repo, services::dashboard_service as service};

#[tauri::command]
pub async fn dashboard_snapshot(db: State<'_, ClientDb>) -> Result<repo::DashboardSnapshot, String> {
    service::snapshot(&db.0).await
}

#[tauri::command]
pub async fn dashboard_record_baselines(db: State<'_, ClientDb>, bauwerke: Vec<Value>) -> Result<usize, String> {
    service::record_downloaded(&db.0, bauwerke).await
}

#[tauri::command]
pub async fn dashboard_record_server_check(db: State<'_, ClientDb>, bauwerke: Vec<Value>) -> Result<(), String> {
    service::record_server_check(&db.0, bauwerke).await
}

#[tauri::command]
pub async fn dashboard_record_opened(db: State<'_, ClientDb>, id: i64) -> Result<(), String> {
    service::record_opened(&db.0, id).await
}
