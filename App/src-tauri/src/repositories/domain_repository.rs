//! SeaORM Entity-first persistence for the whitelisted client domain.
//!
//! Ordinary CRUD and M:N relationships use generated SeaORM entities, not
//! hand-written SELECT/INSERT/UPDATE/DELETE statements. JSON is translated at
//! the IPC boundary; adding a column to a migration and entity does not require
//! maintaining per-table SQL projection or update strings.
//!
//! Two deliberate SQLite-specific exceptions remain:
//! - PRAGMA table_info retrieves column defaults needed by the existing Django
//!   compatible generic form API, without a hardcoded list of fields.
//! - Media deletion nulls polymorphic references which are not modeled as
//!   regular foreign-key relations in the server-derived schema.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend,
    EntityTrait, IdenStatic, IntoActiveModel, Iterable, QueryFilter, QueryOrder,
    Statement, TransactionTrait, TryIntoModel, Value as DbValue,
};
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{entities, services::domain_service::{api_to_column_value, column_to_api, json_field, DomainResource, Relation}};

#[derive(Clone)]
struct ColumnInfo {
    name: String,
    sql_type: String,
    not_null: bool,
    has_default: bool,
    primary_key: bool,
}

fn quote(s: &str) -> String { format!("\"{}\"", s.replace('"', "\"\"")) }
fn stmt(sql: String, args: Vec<DbValue>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Sqlite, sql, args)
}

// Schema introspection only. The field list is not maintained by developers.
async fn columns<C: ConnectionTrait>(conn: &C, table: &str) -> Result<Vec<ColumnInfo>, String> {
    let rows = conn.query_all_raw(Statement::from_string(
        DbBackend::Sqlite, format!("PRAGMA table_info({})", quote(table))
    )).await.map_err(|e| e.to_string())?;
    rows.iter().map(|row| Ok(ColumnInfo {
        name: row.try_get("", "name").map_err(|e| e.to_string())?,
        sql_type: row.try_get("", "type").map_err(|e| e.to_string())?,
        not_null: row.try_get::<i64>("", "notnull").map_err(|e| e.to_string())? != 0,
        has_default: row.try_get::<Option<String>>("", "dflt_value")
            .map_err(|e| e.to_string())?.is_some(),
        primary_key: row.try_get::<i64>("", "pk").map_err(|e| e.to_string())? != 0,
    })).collect()
}

fn default_value(col: &ColumnInfo) -> Option<Value> {
    if col.primary_key || !col.not_null || col.has_default { return None; }
    if col.name.ends_with("_id") || col.name.ends_with("_id_id") { return None; }
    if col.name == "datum" {
        return Some(Value::String(chrono::Utc::now().format("%Y-%m-%d").to_string()));
    }
    if matches!(col.name.as_str(), "hochgeladen_am" | "zuletzt_geaendert_am") {
        return Some(Value::String(chrono::Utc::now().naive_utc()
            .format("%Y-%m-%dT%H:%M:%S%.f").to_string()));
    }
    let t = col.sql_type.to_uppercase();
    if t.contains("BOOL") { Some(Value::Bool(false)) }
    else if t.contains("INT") { Some(Value::from(0)) }
    else if t.contains("REAL") || t.contains("FLOAT") { Some(Value::from(0.0)) }
    else { Some(Value::String(String::new())) }
}

fn normalize(value: Value, col: &ColumnInfo) -> Result<Value, String> {
    if json_field(&col.name) { return Ok(value); }
    let t = col.sql_type.to_uppercase();
    // Django commonly sends timezone-qualified timestamps; SeaORM's DateTime
    // entity fields use chrono::NaiveDateTime (UTC without an offset).
    if t.contains("DATETIME") {
        if let Value::String(text) = &value {
            if let Ok(timestamp) = chrono::DateTime::parse_from_rfc3339(text) {
                return Ok(Value::String(timestamp.naive_utc()
                    .format("%Y-%m-%dT%H:%M:%S%.f").to_string()));
            }
        }
    }
    if value == "" && (t.contains("INT") || t.contains("REAL") || t.contains("FLOAT")) {
        return Ok(Value::Null);
    }
    if t.contains("BOOL") {
        if let Some(n) = value.as_i64() { return Ok(Value::Bool(n != 0)); }
    }
    if value.is_array() || value.is_object() {
        return Err(format!("Unsupported complex value for {}", col.name));
    }
    Ok(value)
}

// Converts Django's FK names and actual SQLite names to the model's serde keys.
// No per-entity column mapping is maintained: Column::iter() supplies the list.
async fn active_input<E, C>(conn: &C, table: &str, payload: &Map<String, Value>,
    creating: bool, importing: bool) -> Result<Value, String>
where E: EntityTrait, C: ConnectionTrait {
    let cols = columns(conn, table).await?;
    let mut out = Map::new();
    for col in E::Column::iter() {
        let db_name = col.as_str();
        let Some(info) = cols.iter().find(|c| c.name == db_name) else {
            return Err(format!("Entity/schema mismatch: {table}.{db_name}"));
        };
        if info.primary_key && creating && !importing { continue; }
        let supplied = api_to_column_value(payload, db_name).cloned()
            .or_else(|| if creating && !importing { default_value(info) } else { None });
        if let Some(v) = supplied {
            out.insert(col.json_key().to_owned(), normalize(v, info)?);
        }
    }
    Ok(Value::Object(out))
}

fn entity_column<E: EntityTrait>(name: &str) -> Result<E::Column, String> {
    E::Column::iter().find(|col| col.as_str() == name)
        .ok_or_else(|| format!("Invalid entity column: {name}"))
}

fn entity_json<E: EntityTrait>(model: &E::Model) -> Result<Value, String>
where E::Model: Serialize {
    let data = serde_json::to_value(model).map_err(|e| e.to_string())?;
    let fields = data.as_object().ok_or("SeaORM model must serialize to an object")?;
    let mut out = Map::new();
    for col in E::Column::iter() {
        let val = fields.get(col.json_key()).cloned().unwrap_or(Value::Null);
        out.insert(column_to_api(col.as_str()), val);
    }
    Ok(Value::Object(out))
}

async fn find_rows<E, C>(conn: &C, filters: &[(String, Value)]) -> Result<Vec<Value>, String>
where E: EntityTrait, E::Model: Serialize, C: ConnectionTrait {
    let mut finder = E::find();
    for (name, val) in filters {
        // In a generic API the caller must not be allowed to inject a column.
        let col = entity_column::<E>(name)?;
        let arg: DbValue = match val {
            Value::Bool(v) => (*v).into(),
            Value::Number(n) => if let Some(v) = n.as_i64() { v.into() }
                else { n.as_f64().ok_or("Invalid filter number")?.into() },
            Value::String(v) => v.clone().into(),
            Value::Null => DbValue::String(None),
            _ => return Err(format!("Invalid filter value for {name}")),
        };
        finder = finder.filter(col.eq(arg));
    }
    if let Ok(order) = entity_column::<E>("sort_order") {
        finder = finder.order_by_asc(order);
    }
    finder = finder.order_by_asc(entity_column::<E>("id")?);
    finder.all(conn).await.map_err(|e| e.to_string())?
        .iter().map(entity_json::<E>).collect()
}

async fn find_one<E, C>(conn: &C, id: i64) -> Result<Option<Value>, String>
where E: EntityTrait, E::Model: Serialize, C: ConnectionTrait {
    let record = E::find().filter(entity_column::<E>("id")?.eq(id))
        .one(conn).await.map_err(|e| e.to_string())?;
    record.as_ref().map(entity_json::<E>).transpose()
}

async fn write_row<E, C>(conn: &C, spec: DomainResource, payload: &Value,
    update_id: Option<i64>, importing: bool) -> Result<i64, String>
where E: EntityTrait, C: ConnectionTrait,
      E::Model: Serialize + serde::de::DeserializeOwned + IntoActiveModel<E::ActiveModel>,
      E::ActiveModel: TryIntoModel<E::Model> + Send {
    let map = payload.as_object().ok_or("Expected object payload")?;
    let id_col = entity_column::<E>("id")?;
    if let Some(id) = update_id {
        if E::find().filter(id_col.eq(id)).one(conn).await.map_err(|e| e.to_string())?.is_none() {
            return Err(format!("Record {id} not found"));
        }
        let fields = active_input::<E, C>(conn, spec.table, map, false, false).await?;
        if fields.as_object().is_some_and(|m| m.is_empty()) { return Ok(id); }
        let mut active = E::ActiveModel::from_json(fields).map_err(|e| e.to_string())?;
        active.try_set(id_col, id.into()).map_err(|e| e.to_string())?;
        active.update(conn).await.map_err(|e| e.to_string())?;
        Ok(id)
    } else {
        let explicit_id = if importing {
            Some(map.get("id").and_then(Value::as_i64).ok_or("Import requires an id")?)
        } else { None };
        if let Some(id) = explicit_id {
            // Preserve locally edited records during one-way initial import.
            if E::find().filter(id_col.eq(id)).one(conn).await.map_err(|e| e.to_string())?.is_some() {
                return Ok(id);
            }
        }
        let fields = active_input::<E, C>(conn, spec.table, map, true, importing).await?;
        let active = E::ActiveModel::from_json(fields).map_err(|e| e.to_string())?;
        let saved = active.insert(conn).await.map_err(|e| e.to_string())?;
        let id = entity_json::<E>(&saved)?["id"].as_i64().ok_or("Saved record lacks id")?;
        Ok(id)
    }
}

async fn delete_row<E, C>(conn: &C, id: i64) -> Result<(), String>
where E: EntityTrait, C: ConnectionTrait {
    let result = E::delete_many().filter(entity_column::<E>("id")?.eq(id))
        .exec(conn).await.map_err(|e| e.to_string())?;
    if result.rows_affected == 0 { return Err(format!("Record {id} not found")); }
    Ok(())
}

async fn link_ids<E, C>(conn: &C, rel: &Relation, owner_id: i64) -> Result<Vec<Value>, String>
where E: EntityTrait, E::Model: Serialize, C: ConnectionTrait {
    let list = E::find().filter(entity_column::<E>(rel.parent_key)?.eq(owner_id))
        .all(conn).await.map_err(|e| e.to_string())?;
    let target_col = entity_column::<E>(rel.target_key)?;
    list.iter().map(|item| {
        let row = serde_json::to_value(item).map_err(|e| e.to_string())?;
        row.get(target_col.json_key()).cloned().ok_or_else(|| "Missing linked target".to_owned())
    }).collect()
}

async fn write_links<E, C>(conn: &C, rel: &Relation, owner_id: i64,
    ids: &[Value], replace: bool) -> Result<(), String>
where E: EntityTrait, C: ConnectionTrait,
      E::Model: Serialize + serde::de::DeserializeOwned + IntoActiveModel<E::ActiveModel>,
      E::ActiveModel: TryIntoModel<E::Model> + Send {
    let owner_col = entity_column::<E>(rel.parent_key)?;
    let target_col = entity_column::<E>(rel.target_key)?;
    if replace {
        E::delete_many().filter(owner_col.eq(owner_id)).exec(conn)
            .await.map_err(|e| e.to_string())?;
    }
    for item in ids {
        let other_id = item.as_i64().ok_or("Expected numeric foreign key")?;
        // Avoid duplicate M:N rows, especially during repeated initial imports.
        let exists = E::find().filter(owner_col.eq(owner_id)).filter(target_col.eq(other_id))
            .one(conn).await.map_err(|e| e.to_string())?.is_some();
        if !exists {
            let mut obj = Map::new();
            obj.insert(owner_col.json_key().to_string(), Value::from(owner_id));
            obj.insert(target_col.json_key().to_string(), Value::from(other_id));
            E::ActiveModel::from_json(Value::Object(obj)).map_err(|e| e.to_string())?
                .insert(conn).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

async fn delete_media_links<E, C>(conn: &C, id: i64) -> Result<(), String>
where E: EntityTrait, C: ConnectionTrait {
    E::delete_many().filter(entity_column::<E>("media_id")?.eq(id))
        .exec(conn).await.map_err(|e| e.to_string())?;
    Ok(())
}

macro_rules! with_domain {
    ($table:expr, $function:ident, $($args:expr),*) => {{
        match $table {
            "assetverwaltung_bauwerk" => $function::<entities::assetverwaltung_bauwerk::Entity, _>($($args),*).await,
            "assetverwaltung_projekt" => $function::<entities::assetverwaltung_projekt::Entity, _>($($args),*).await,
            "assetverwaltung_person" => $function::<entities::assetverwaltung_person::Entity, _>($($args),*).await,
            "assetverwaltung_firma" => $function::<entities::assetverwaltung_firma::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_untersuchungsstelle" => $function::<entities::bauwerksdiagnostik_untersuchungsstelle::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_bohrkern" => $function::<entities::bauwerksdiagnostik_bohrkern::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_probe" => $function::<entities::bauwerksdiagnostik_probe::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_material" => $function::<entities::bauwerksdiagnostik_material::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_media" => $function::<entities::bauwerksdiagnostik_media::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_rissbreitenmessung" => $function::<entities::bauwerksdiagnostik_rissbreitenmessung::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_rueckprallhammer" => $function::<entities::bauwerksdiagnostik_rueckprallhammer::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_kernbohrung" => $function::<entities::bauwerksdiagnostik_kernbohrung::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_chloridanalyse" => $function::<entities::bauwerksdiagnostik_chloridanalyse::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_druckfestigkeitspruefung" => $function::<entities::bauwerksdiagnostik_druckfestigkeitspruefung::Entity, _>($($args),*).await,
            _ => Err(format!("Unsupported domain entity {table}", table=$table)),
        }
    }};
}

macro_rules! with_join {
    ($table:expr, $function:ident, $($args:expr),*) => {{
        match $table {
            "assetverwaltung_bauwerk_projekt" => $function::<entities::assetverwaltung_bauwerk_projekt::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_untersuchungsstelle_media" => $function::<entities::bauwerksdiagnostik_untersuchungsstelle_media::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_bohrkern_fotos" => $function::<entities::bauwerksdiagnostik_bohrkern_fotos::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_bohrkern_material" => $function::<entities::bauwerksdiagnostik_bohrkern_material::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_probe_foto" => $function::<entities::bauwerksdiagnostik_probe_foto::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_probe_material" => $function::<entities::bauwerksdiagnostik_probe_material::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_rissbreitenmessung_bearbeiter" => $function::<entities::bauwerksdiagnostik_rissbreitenmessung_bearbeiter::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_rissbreitenmessung_bilder" => $function::<entities::bauwerksdiagnostik_rissbreitenmessung_bilder::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_rueckprallhammer_bearbeiter" => $function::<entities::bauwerksdiagnostik_rueckprallhammer_bearbeiter::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_rueckprallhammer_bilder" => $function::<entities::bauwerksdiagnostik_rueckprallhammer_bilder::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_rueckprallhammer_messprotokolle" => $function::<entities::bauwerksdiagnostik_rueckprallhammer_messprotokolle::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_kernbohrung_bearbeiter" => $function::<entities::bauwerksdiagnostik_kernbohrung_bearbeiter::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_kernbohrung_bilder" => $function::<entities::bauwerksdiagnostik_kernbohrung_bilder::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_chloridanalyse_bearbeiter" => $function::<entities::bauwerksdiagnostik_chloridanalyse_bearbeiter::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_chloridanalyse_bilder" => $function::<entities::bauwerksdiagnostik_chloridanalyse_bilder::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_chloridanalyse_pruefberichte" => $function::<entities::bauwerksdiagnostik_chloridanalyse_pruefberichte::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter" => $function::<entities::bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter::Entity, _>($($args),*).await,
            "bauwerksdiagnostik_druckfestigkeitspruefung_bilder" => $function::<entities::bauwerksdiagnostik_druckfestigkeitspruefung_bilder::Entity, _>($($args),*).await,
            _ => Err(format!("Unsupported junction entity {table}", table=$table)),
        }
    }};
}

async fn attach_links<C: ConnectionTrait>(conn: &C, spec: DomainResource, row: &mut Value)
    -> Result<(), String> {
    let Some(id) = row.get("id").and_then(Value::as_i64) else { return Ok(()); };
    for rel in spec.relations {
        let ids = with_join!(rel.table, link_ids, conn, rel, id)?;
        row.as_object_mut().ok_or("Row must be object")?
            .insert(rel.property.into(), Value::Array(ids));
    }
    Ok(())
}

async fn set_links<C: ConnectionTrait>(conn: &C, spec: DomainResource, id: i64,
    input: &Map<String, Value>, replace: bool) -> Result<(), String> {
    for rel in spec.relations {
        if let Some(value) = input.get(rel.property) {
            let ids = value.as_array().ok_or_else(|| format!("{} must be an array", rel.property))?;
            with_join!(rel.table, write_links, conn, rel, id, ids, replace)?;
        }
    }
    Ok(())
}

async fn media_child_exists<C: ConnectionTrait>(conn: &C, id: i64, kind: &str) -> Result<bool, String> {
    let exists = match kind {
        "image" => entities::bauwerksdiagnostik_imagemedia::Entity::find()
            .filter(entities::bauwerksdiagnostik_imagemedia::Column::MediaPtrId.eq(id))
            .one(conn).await.map_err(|e| e.to_string())?.is_some(),
        "document" => entities::bauwerksdiagnostik_documentmedia::Entity::find()
            .filter(entities::bauwerksdiagnostik_documentmedia::Column::MediaPtrId.eq(id))
            .one(conn).await.map_err(|e| e.to_string())?.is_some(),
        "messdatei" => entities::bauwerksdiagnostik_messdateimedia::Entity::find()
            .filter(entities::bauwerksdiagnostik_messdateimedia::Column::MediaPtrId.eq(id))
            .one(conn).await.map_err(|e| e.to_string())?.is_some(),
        _ => false,
    };
    Ok(exists)
}

async fn load_child<E, C>(conn: &C, id: i64) -> Result<Option<Value>, String>
where E: EntityTrait, E::Model: Serialize, C: ConnectionTrait {
    let col = entity_column::<E>("media_ptr_id")?;
    let row = E::find().filter(col.eq(id)).one(conn)
        .await.map_err(|e| e.to_string())?;
    row.as_ref().map(entity_json::<E>).transpose()
}

async fn save_child<E, C>(conn: &C, id: i64, payload: &Value, kind: &str) -> Result<(), String>
where E: EntityTrait, C: ConnectionTrait,
      E::Model: Serialize + serde::de::DeserializeOwned + IntoActiveModel<E::ActiveModel>,
      E::ActiveModel: TryIntoModel<E::Model> + Send {
    let map = payload.as_object().ok_or("Expected media object")?;
    let table = match kind {
        "image" => "bauwerksdiagnostik_imagemedia",
        "document" => "bauwerksdiagnostik_documentmedia",
        _ => "bauwerksdiagnostik_messdateimedia",
    };
    let mut fields = active_input::<E, C>(conn, table, map, false, false).await?;
    // When imported server media lacks fields, honor the same local defaults.
    let has_row = E::find().filter(entity_column::<E>("media_ptr_id")?.eq(id))
        .one(conn).await.map_err(|e| e.to_string())?.is_some();
    let mut data = fields.as_object_mut().ok_or("Media fields must be an object")?.clone();
    data.insert(entity_column::<E>("media_ptr_id")?.json_key().into(), Value::from(id));
    if !has_row {
        let schema = columns(conn, table).await?;
        for column in E::Column::iter() {
            if let Some(info) = schema.iter().find(|col| col.name == column.as_str()) {
                if !data.contains_key(column.json_key()) {
                    if let Some(default) = default_value(info) {
                        data.insert(column.json_key().into(), default);
                    }
                }
            }
        }
        E::ActiveModel::from_json(Value::Object(data)).map_err(|e| e.to_string())?
            .insert(conn).await.map_err(|e| e.to_string())?;
    } else if data.len() > 1 {
        let mut active = E::ActiveModel::from_json(Value::Object(data))
            .map_err(|e| e.to_string())?;
        active.update(conn).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

async fn media_child_load<C: ConnectionTrait>(conn: &C, id: i64, row: &mut Value) -> Result<(), String> {
    for (kind, child) in [
        ("image", load_child::<entities::bauwerksdiagnostik_imagemedia::Entity, C>(conn, id).await?),
        ("document", load_child::<entities::bauwerksdiagnostik_documentmedia::Entity, C>(conn, id).await?),
        ("messdatei", load_child::<entities::bauwerksdiagnostik_messdateimedia::Entity, C>(conn, id).await?),
    ] {
        if let Some(Value::Object(data)) = child {
            let obj = row.as_object_mut().ok_or("Row must be object")?;
            for (key, val) in data { if key != "media_ptr" && key != "media_ptr_id" { obj.insert(key, val); } }
            obj.insert("media_type".into(), Value::String(kind.into()));
            return Ok(());
        }
    }
    row.as_object_mut().ok_or("Row must be object")?
        .insert("media_type".into(), Value::String("media".into()));
    Ok(())
}

async fn media_child_save<C: ConnectionTrait>(conn: &C, id: i64, payload: &Value) -> Result<(), String> {
    let kind = payload.get("media_type").and_then(Value::as_str).unwrap_or("").to_lowercase();
    let chosen = if matches!(kind.as_str(), "image" | "document" | "messdatei") { kind }
        else {
            let mut existing = None;
            for candidate in ["image", "document", "messdatei"] {
                if media_child_exists(conn, id, candidate).await? { existing = Some(candidate.to_string()); break; }
            }
            let Some(kind) = existing else { return Ok(()); };
            kind
        };
    match chosen.as_str() {
        "image" => save_child::<entities::bauwerksdiagnostik_imagemedia::Entity, C>(conn,id,payload,"image").await,
        "document" => save_child::<entities::bauwerksdiagnostik_documentmedia::Entity, C>(conn,id,payload,"document").await,
        _ => save_child::<entities::bauwerksdiagnostik_messdateimedia::Entity, C>(conn,id,payload,"messdatei").await,
    }
}

pub async fn list(db: &DatabaseConnection, spec: DomainResource,
    filters: Vec<(String, Value)>) -> Result<Vec<Value>, String> {
    let mut rows = with_domain!(spec.table, find_rows, db, &filters)?;
    for row in &mut rows {
        attach_links(db, spec, row).await?;
        if spec.table == "bauwerksdiagnostik_media" {
            if let Some(id) = row.get("id").and_then(Value::as_i64) {
                media_child_load(db, id, row).await?;
            }
        }
    }
    Ok(rows)
}

pub async fn get(db: &DatabaseConnection, spec: DomainResource, id: i64)
    -> Result<Value, String> {
    let mut row = with_domain!(spec.table, find_one, db, id)?
        .ok_or_else(|| format!("Record {id} not found"))?;
    attach_links(db, spec, &mut row).await?;
    if spec.table == "bauwerksdiagnostik_media" { media_child_load(db, id, &mut row).await?; }
    Ok(row)
}

pub async fn create(db: &DatabaseConnection, spec: DomainResource, payload: Value)
    -> Result<Value, String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    let id = with_domain!(spec.table, write_row, &txn, spec, &payload, None, false)?;
    if spec.table == "bauwerksdiagnostik_media" { media_child_save(&txn, id, &payload).await?; }
    set_links(&txn, spec, id, payload.as_object().ok_or("Expected object")?, true).await?;
    txn.commit().await.map_err(|e| e.to_string())?;
    get(db, spec, id).await
}

pub async fn update(db: &DatabaseConnection, spec: DomainResource, id: i64, payload: Value)
    -> Result<Value, String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    with_domain!(spec.table, write_row, &txn, spec, &payload, Some(id), false)?;
    if spec.table == "bauwerksdiagnostik_media" { media_child_save(&txn, id, &payload).await?; }
    set_links(&txn, spec, id, payload.as_object().ok_or("Expected object")?, true).await?;
    txn.commit().await.map_err(|e| e.to_string())?;
    get(db, spec, id).await
}

pub async fn delete(db: &DatabaseConnection, spec: DomainResource, id: i64)
    -> Result<(), String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    // Dashboard metadata references the domain Bauwerk. Remove its dependents
    // inside the same transaction before deleting the domain record.
    if spec.table == "assetverwaltung_bauwerk" {
        entities::client_dashboard_bauwerk_baseline::Entity::delete_by_id(id)
            .exec(&txn).await.map_err(|e| e.to_string())?;
        entities::client_dashboard_recent_bauwerk::Entity::delete_by_id(id)
            .exec(&txn).await.map_err(|e| e.to_string())?;
    }
    for rel in spec.relations {
        with_join!(rel.table, erase_parent_links, &txn, rel, id)?;
    }
    if spec.table == "bauwerksdiagnostik_media" {
        for table in [
            "bauwerksdiagnostik_bohrkern_fotos",
            "bauwerksdiagnostik_chloridanalyse_bilder",
            "bauwerksdiagnostik_chloridanalyse_pruefberichte",
            "bauwerksdiagnostik_druckfestigkeitspruefung_bilder",
            "bauwerksdiagnostik_kernbohrung_bilder",
            "bauwerksdiagnostik_probe_foto",
            "bauwerksdiagnostik_rissbreitenmessung_bilder",
            "bauwerksdiagnostik_rueckprallhammer_bilder",
            "bauwerksdiagnostik_rueckprallhammer_messprotokolle",
            "bauwerksdiagnostik_untersuchungsstelle_media",
        ] {
            with_join!(table, delete_media_links, &txn, id)?;
        }
        // A few Django polymorphic references have no modeled relation in SeaORM.
        // Keep their nulling narrowly isolated; never expose user-provided SQL names.
        for table in [
            "bauwerksdiagnostik_chloridanalyse",
            "bauwerksdiagnostik_druckfestigkeitspruefung",
            "bauwerksdiagnostik_kernbohrung",
            "bauwerksdiagnostik_rissbreitenmessung",
            "bauwerksdiagnostik_rueckprallhammer",
        ] {
            txn.execute_raw(stmt(
                format!("UPDATE {table} SET titelBild_id=NULL WHERE titelBild_id=?"),
                vec![id.into()],
            )).await.map_err(|e| e.to_string())?;
        }
        txn.execute_raw(stmt(
            "UPDATE bauwerksdiagnostik_druckfestigkeitspruefung SET pruefbericht_id=NULL WHERE pruefbericht_id=?".into(),
            vec![id.into()],
        )).await.map_err(|e| e.to_string())?;
        for kind in ["image", "document", "messdatei"] {
            match kind {
                "image" => { entities::bauwerksdiagnostik_imagemedia::Entity::delete_many()
                    .filter(entities::bauwerksdiagnostik_imagemedia::Column::MediaPtrId.eq(id))
                    .exec(&txn).await.map_err(|e| e.to_string())?; }
                "document" => { entities::bauwerksdiagnostik_documentmedia::Entity::delete_many()
                    .filter(entities::bauwerksdiagnostik_documentmedia::Column::MediaPtrId.eq(id))
                    .exec(&txn).await.map_err(|e| e.to_string())?; }
                _ => { entities::bauwerksdiagnostik_messdateimedia::Entity::delete_many()
                    .filter(entities::bauwerksdiagnostik_messdateimedia::Column::MediaPtrId.eq(id))
                    .exec(&txn).await.map_err(|e| e.to_string())?; }
            }
        }
    }
    with_domain!(spec.table, delete_row, &txn, id)?;
    txn.commit().await.map_err(|e| e.to_string())
}

async fn erase_parent_links<E, C>(conn: &C, rel: &Relation, id: i64) -> Result<(), String>
where E: EntityTrait, C: ConnectionTrait {
    E::delete_many().filter(entity_column::<E>(rel.parent_key)?.eq(id))
        .exec(conn).await.map_err(|e| e.to_string())?;
    Ok(())
}

// One-way initial import: preserve local rows even when server IDs already exist.
pub async fn import(db: &DatabaseConnection, spec: DomainResource, records: Vec<Value>)
    -> Result<usize, String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    for record in &records {
        // One-way preparation must never overwrite an existing local record,
        // including its media subtype/annotations. Check before insertion.
        let incoming_id = record.get("id").and_then(Value::as_i64)
            .ok_or("Import requires an id")?;
        if with_domain!(spec.table, find_one, &txn, incoming_id)?.is_some() {
            continue;
        }
        let id = with_domain!(spec.table, write_row, &txn, spec, record, None, true)?;
        if spec.table == "bauwerksdiagnostik_media" {
            media_child_save(&txn, id, record).await?;
        }
    }
    txn.commit().await.map_err(|e| e.to_string())?;
    Ok(records.len())
}

pub async fn import_links(db: &DatabaseConnection, spec: DomainResource, records: Vec<Value>)
    -> Result<(), String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    for record in records {
        let obj = record.as_object().ok_or("Expected relation record")?;
        let id = obj.get("id").and_then(Value::as_i64)
            .ok_or("Relation import requires id")?;
        set_links(&txn, spec, id, obj, false).await?;
    }
    txn.commit().await.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn entity_field_projection_uses_api_names() {
        let row = entities::bauwerksdiagnostik_untersuchungsstelle::Model {
            id: 1,
            name: "Stelle A".into(),
            bemerkung: String::new(),
            lagenbezug_hoehe: Some("Referenz".into()),
            hoehe: None,
            lagenbezug_abstand1: None,
            abstand1: None,
            lagenbezug_abstand2: None,
            abstand2: None,
            bauwerk_id: 10,
            projekt_id: Some(20),
        };
        let api = entity_json::<entities::bauwerksdiagnostik_untersuchungsstelle::Entity>(&row).unwrap();
        assert_eq!(api["bauwerk"], json!(10));
        assert_eq!(api["projekt"], json!(20));
        assert_eq!(api["lagenbezugHoehe"], json!("Referenz"));
    }

    #[test]
    fn partial_active_model_preserves_missing_columns() {
        let model = entities::assetverwaltung_bauwerk::ActiveModel::from_json(
            json!({"bezeichnung": "Brücke 1"})
        ).unwrap();
        assert!(model.is_not_set(entities::assetverwaltung_bauwerk::Column::Archived));
        assert!(model.is_not_set(entities::assetverwaltung_bauwerk::Column::Id));
    }

    #[test]
    fn entity_mapping_tracks_special_database_column_names() {
        let col = entity_column::<entities::bauwerksdiagnostik_rissbreitenmessung::Entity>("maxRissbreite")
            .unwrap();
        assert_eq!(col.as_str(), "maxRissbreite");
        assert_eq!(col.json_key(), "max_rissbreite");
        assert_eq!(column_to_api("bauwerk_id_id"), "bauwerk_id");
        assert_eq!(column_to_api("titelBild_id"), "titelBild");
    }

    #[test]
    fn json_fields_keep_structured_data() {
        let col = ColumnInfo {
            name: "risse".into(), sql_type: "TEXT".into(),
            not_null: false, has_default: false, primary_key: false,
        };
        assert_eq!(normalize(json!([{"measurement": 0.2}]), &col).unwrap(),
            json!([{"measurement": 0.2}]));
    }

    #[test]
    fn entity_crud_relations_and_import_on_real_sqlite() {
        // Tests the full SeaORM repository against the versioned Step 2 schema.
        // Single connection keeps the in-memory SQLite database stable.
        tauri::async_runtime::block_on(async {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .expect("SQLite in-memory pool");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .expect("Step 2 migrations");
            let db = sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
            let projekt_spec = crate::services::domain_service::resource("Projekt").unwrap();
            let bauwerk_spec = crate::services::domain_service::resource("Bauwerk").unwrap();

            let projekt = create(&db, projekt_spec, json!({
                "name": "Testprojekt", "projektnummer": "T-1", "bezeichnung": "Test"
            })).await.expect("ORM create project");
            let projekt_id = projekt["id"].as_i64().unwrap();
            let bauwerk = create(&db, bauwerk_spec, json!({
                "bezeichnung": "Testbauwerk", "projekt": [projekt_id]
            })).await.expect("ORM create Bauwerk with relation");
            let bauwerk_id = bauwerk["id"].as_i64().unwrap();
            assert_eq!(bauwerk["projekt"], json!([projekt_id]));
            assert_eq!(bauwerk["archived"], json!(false));

            update(&db, bauwerk_spec, bauwerk_id, json!({
                "beschreibung": "Offline edited"
            })).await.expect("ORM update Bauwerk");
            let current = get(&db, bauwerk_spec, bauwerk_id).await.unwrap();
            assert_eq!(current["beschreibung"], json!("Offline edited"));
            assert_eq!(current["projekt"], json!([projekt_id]));

            // Import must never overwrite changes already made offline.
            let imported = import(&db, bauwerk_spec, vec![json!({
                "id": bauwerk_id, "beschreibung": "Older server description"
            })]).await.expect("ORM import existing");
            assert_eq!(imported, 1);
            let preserved = get(&db, bauwerk_spec, bauwerk_id).await.unwrap();
            assert_eq!(preserved["beschreibung"], json!("Offline edited"));

            let filtered = list(&db, bauwerk_spec, vec![("id".into(), json!(bauwerk_id))])
                .await.expect("ORM filter by entity id");
            assert_eq!(filtered.len(), 1);
            delete(&db, bauwerk_spec, bauwerk_id).await.expect("ORM delete");
            assert!(get(&db, bauwerk_spec, bauwerk_id).await.is_err());
        });
    }

    #[test]
    fn datetime_from_django_becomes_naive_utc() {
        let col = ColumnInfo {
            name: "hochgeladen_am".to_string(), sql_type: "datetime".into(),
            not_null: true, has_default: false, primary_key: false,
        };
        assert_eq!(normalize(json!("2026-11-15T10:00:00+02:00"), &col).unwrap(),
            json!("2026-11-15T08:00:00"));
    }
}
