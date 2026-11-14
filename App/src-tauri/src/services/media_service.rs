//! Local file bytes are stored beside SQLite in Tauri's application-data folder.
//! They are never uploaded to Django by this module.
use std::{fs, path::PathBuf};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::Value;
use tauri::Manager;
use uuid::Uuid;
use crate::{db::ClientDb, repositories::domain_repository, services::domain_service};

fn media_dir(app: &tauri::AppHandle) -> Result<PathBuf,String> {
    let base = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let dir = base.join("local-media");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}
fn decode(text: &str) -> Result<Vec<u8>, String> {
    // Avoid accidental multi-gigabyte allocations through the Tauri command.
    if text.len() > 128 * 1024 * 1024 { return Err("File exceeds the 96 MB local transfer limit".into()); }
    STANDARD.decode(text).map_err(|e| e.to_string())
}
fn extension(mime: &str) -> &'static str {
    match mime { "image/jpeg" => "jpg", "image/png" => "png", "image/webp" => "webp", "application/pdf" => "pdf", "video/mp4" => "mp4", "text/plain" => "txt", _ => "bin" }
}

pub async fn save(app: &tauri::AppHandle, db: &ClientDb, metadata: Value, content_base64: String, mime: String) -> Result<Value,String> {
    let bytes = decode(&content_base64)?;
    let filename = format!("{}.{}", Uuid::new_v4(), extension(&mime));
    let filepath = media_dir(app)?.join(&filename);
    fs::write(&filepath, &bytes).map_err(|e| e.to_string())?;
    let mut data = metadata.as_object().ok_or("Expected media metadata object")?.clone();
    data.insert("datei".into(), Value::String(format!("local-media/{filename}")));
    data.entry("hochgeladen_von").or_insert(Value::String("Offline client".into()));
    let result = domain_repository::create(&db.0, domain_service::resource("Media")?, Value::Object(data)).await;
    if result.is_err() { let _ = fs::remove_file(&filepath); }
    result
}

pub async fn attach(app: &tauri::AppHandle, db: &ClientDb, id: i64, content_base64: String, mime: String) -> Result<(),String> {
    // Re-running the initial download must not replace an existing local file.
    let old = domain_repository::get(&db.0, domain_service::resource("Media")?, id).await?;
    if let Some(file) = old.get("datei").and_then(Value::as_str).and_then(|s|s.strip_prefix("local-media/")) {
        if !file.contains('/') && !file.contains('\\') && media_dir(app)?.join(file).exists() { return Ok(()); }
    }
    let bytes = decode(&content_base64)?;
    let filename = format!("{}.{}", Uuid::new_v4(), extension(&mime));
    let filepath = media_dir(app)?.join(&filename);
    fs::write(&filepath, &bytes).map_err(|e| e.to_string())?;
    let data = serde_json::json!({"datei": format!("local-media/{filename}")});
    let res = domain_repository::update(&db.0, domain_service::resource("Media")?, id, data).await;
    if res.is_err() { let _ = fs::remove_file(&filepath); }
    res.map(|_| ())
}

pub async fn read(app: &tauri::AppHandle, db: &ClientDb, id: i64) -> Result<String,String> {
    let row = domain_repository::get(&db.0, domain_service::resource("Media")?, id).await?;
    let relative = row.get("datei").and_then(Value::as_str).ok_or("Media has no local file")?;
    let filename = relative.strip_prefix("local-media/").ok_or("This media file has not been downloaded for offline use")?;
    // Files are random UUIDs in one directory; refuse user-controlled path components.
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") { return Err("Invalid local media path".into()); }
    let bytes = fs::read(media_dir(app)?.join(filename)).map_err(|e| e.to_string())?;
    Ok(STANDARD.encode(bytes))
}
