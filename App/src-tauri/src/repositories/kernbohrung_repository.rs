//! Atomic local Bohrkern creation/recovery and Kernbohrung linking.
//! Both records are updated in ONE SeaORM transaction. No handwritten SQL.
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
              Set, TransactionTrait};
use serde_json::Value;
use crate::{entities::{bauwerksdiagnostik_bohrkern as bohrkern,
                       bauwerksdiagnostik_kernbohrung as kernbohrung},
            repositories::domain_repository, services::domain_service};

pub async fn create_or_link(
    db: &DatabaseConnection,
    kernbohrung_id: i64,
    name: &str,
    existing_bohrkern_id: Option<i64>,
) -> Result<Value, String> {
    if kernbohrung_id <= 0 { return Err("Ungültige Kernbohrung-ID".into()); }
    if existing_bohrkern_id.is_none() && name.trim().is_empty() {
        return Err("Bohrkern-Bezeichnung darf nicht leer sein".into());
    }

    let txn = db.begin().await.map_err(|e| e.to_string())?;
    let drilling = kernbohrung::Entity::find_by_id(kernbohrung_id)
        .one(&txn).await.map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Kernbohrung ID {kernbohrung_id} nicht gefunden"))?;
    if drilling.bohrkern_id.is_some() {
        return Err("Dieser Kernbohrung ist bereits ein Bohrkern zugeordnet. Bitte die Ansicht aktualisieren.".into());
    }

    let core = if let Some(existing_id) = existing_bohrkern_id {
        if existing_id <= 0 { return Err("Ungültige Bohrkern-ID".into()); }
        let existing = bohrkern::Entity::find_by_id(existing_id)
            .one(&txn).await.map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Bohrkern ID {existing_id} wurde nicht gefunden"))?;
        let assigned = kernbohrung::Entity::find()
            .filter(kernbohrung::Column::BohrkernId.eq(existing_id))
            .one(&txn).await.map_err(|e| e.to_string())?;
        if assigned.is_some() {
            return Err(format!("Bohrkern ID {existing_id} ist bereits einer Kernbohrung zugeordnet"));
        }
        existing
    } else {
        bohrkern::ActiveModel {
            name: Set(name.trim().to_string()),
            ..Default::default()
        }
        .insert(&txn).await.map_err(|e| e.to_string())?
    };

    let mut active: kernbohrung::ActiveModel = drilling.into();
    active.bohrkern_id = Set(Some(core.id));
    active.update(&txn).await.map_err(|e| e.to_string())?;
    txn.commit().await.map_err(|e| e.to_string())?;

    // Read the result through the existing Entity-first domain repository
    // so the frontend gets the same API field names and material list.
    domain_repository::get(db, domain_service::resource("Bohrkern")?, core.id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::SqlxSqliteConnector;

    #[test]
    fn create_and_link_is_atomic_and_prevents_duplicate_cores() {
        tauri::async_runtime::block_on(async {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1).connect("sqlite::memory:")
                .await.expect("SQLite memory pool");
            sqlx::migrate!("./migrations").run(&pool).await.expect("migrations");
            let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
            let drilling = kernbohrung::ActiveModel {
                datum: Set(chrono::NaiveDate::from_ymd_opt(2026, 11, 16).unwrap()),
                bemerkung: Set(String::new()),
                sort_order: Set(1),
                kuehlung: Set(String::new()),
                verschlussmaterial: Set(String::new()),
                ..Default::default()
            }.insert(&db).await.expect("Kernbohrung");
            // Regression: generic Entity-first CRUD must round-trip field values.
            let spec = domain_service::resource("Kernbohrung").unwrap();
            let updated = domain_repository::update(&db, spec, drilling.id,
                serde_json::json!({
                    "lufttemperatur": 18.5,
                    "luftfeuchte": 53.0,
                    "verschlussmaterial": "Mörtel"
                })
            ).await.expect("update measurements");
            assert_eq!(updated["lufttemperatur"], serde_json::json!(18.5));
            assert_eq!(updated["luftfeuchte"], serde_json::json!(53.0));
            assert_eq!(updated["verschlussmaterial"], serde_json::json!("Mörtel"));
            let reread_fields = domain_repository::get(&db, spec, drilling.id)
                .await.expect("reload measurements");
            assert_eq!(reread_fields["verschlussmaterial"], serde_json::json!("Mörtel"));
            let created = create_or_link(&db, drilling.id, "BK-1", None).await.expect("atomic creation");
            let id = created["id"].as_i64().expect("core id");
            let reread = kernbohrung::Entity::find_by_id(drilling.id)
                .one(&db).await.expect("read drilling").expect("drilling exists");
            assert_eq!(reread.bohrkern_id, Some(id));
            assert!(create_or_link(&db, drilling.id, "BK-2", None).await.is_err());
            assert_eq!(bohrkern::Entity::find().all(&db).await.unwrap().len(), 1);
        });
    }
}
