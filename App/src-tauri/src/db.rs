//! Client-side SQLite initialization and SeaORM connection.
//! No Django API calls are made here, and no server schema is altered.

use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, SqlxSqliteConnector};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use std::{fs, time::Duration};
use tauri::Manager;

/// Managed by Tauri for later read/write commands.
pub struct ClientDb(pub DatabaseConnection);

/// Open/create a per-installation SQLite database and apply new migrations.
pub async fn initialize(app: &tauri::AppHandle) -> Result<ClientDb, String> {
    let directory = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    let path = directory.join("gate-offline.sqlite3");

    let options = SqliteConnectOptions::new()
        .filename(&path)
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| format!("Cannot open client SQLite: {e}"))?;

    // SQLx records applied migrations in _sqlx_migrations; it won't repeat them.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| format!("Client SQLite migration failed: {e}"))?;

    let connection = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);

    // First actual ORM query: the database is initially empty, and that's OK.
    let count = crate::entities::assetverwaltung_projekt::Entity::find()
        .count(&connection)
        .await
        .map_err(|e| format!("SeaORM self-test failed: {e}"))?;

    println!("[GATE] Client SQLite: {}", path.display());
    println!("[GATE] SeaORM ready. Local projects: {count}");
    Ok(ClientDb(connection))
}
