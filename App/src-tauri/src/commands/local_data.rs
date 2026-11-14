//! Tauri IPC boundary: no database SQL or business rules here.
use serde_json::Value;
use tauri::State;
use crate::{db::ClientDb, repositories::domain_repository as repo, services::{domain_service, media_service}};

#[tauri::command]
pub async fn local_list(db: State<'_, ClientDb>, resource: String, filters: Option<Value>) -> Result<Vec<Value>,String> {
    let spec = domain_service::resource(&resource)?;
    let mut where_fields = Vec::new();
    if let Some(Value::Object(map)) = filters {
        for (name, val) in map {
            if val.is_null() { continue; }
            let column = if resource.eq_ignore_ascii_case("Probe") && name == "bauwerk" { "bauwerk_id_id".to_string() }
                else if resource.eq_ignore_ascii_case("Probe") && name == "projekt" { "projekt_id_id".to_string() }
                else if name.ends_with("_id") { name } else { format!("{name}_id") };
            where_fields.push((column, val));
        }
    }
    repo::list(&db.0, spec, where_fields).await
}
#[tauri::command]
pub async fn local_get(db: State<'_, ClientDb>, resource: String, id: i64) -> Result<Value,String> {
    repo::get(&db.0, domain_service::resource(&resource)?, id).await
}
#[tauri::command]
pub async fn local_create(db: State<'_, ClientDb>, resource: String, data: Value) -> Result<Value,String> {
    let spec = domain_service::resource(&resource)?;
    let validated = domain_service::prepare_for_save(&db.0, spec, data, true).await?;
    repo::create(&db.0, spec, validated).await
}
#[tauri::command]
pub async fn local_update(db: State<'_, ClientDb>, resource: String, id: i64, data: Value) -> Result<Value,String> {
    let spec = domain_service::resource(&resource)?;
    let validated = domain_service::prepare_for_save(&db.0, spec, data, false).await?;
    repo::update(&db.0, spec, id, validated).await
}
#[tauri::command]
pub async fn local_delete(db: State<'_, ClientDb>, resource: String, id: i64) -> Result<(),String> {
    repo::delete(&db.0, domain_service::resource(&resource)?, id).await
}
#[tauri::command]
pub async fn local_import(db: State<'_, ClientDb>, resource: String, records: Vec<Value>) -> Result<usize,String> {
    repo::import(&db.0, domain_service::resource(&resource)?, records).await
}
#[tauri::command]
pub async fn local_import_links(db: State<'_, ClientDb>, resource: String, records: Vec<Value>) -> Result<(),String> {
    repo::import_links(&db.0, domain_service::resource(&resource)?, records).await
}
#[tauri::command]
pub async fn local_save_media(app: tauri::AppHandle, db: State<'_, ClientDb>, metadata: Value, content_base64: String, mime: String) -> Result<Value,String> {
    media_service::save(&app, &db, metadata, content_base64, mime).await
}
#[tauri::command]
pub async fn local_attach_media(app: tauri::AppHandle, db: State<'_, ClientDb>, id: i64, content_base64: String, mime: String) -> Result<(),String> {
    media_service::attach(&app, &db, id, content_base64, mime).await
}
#[tauri::command]
pub async fn local_read_media(app: tauri::AppHandle, db: State<'_, ClientDb>, id: i64) -> Result<String,String> {
    media_service::read(&app, &db, id).await
}
