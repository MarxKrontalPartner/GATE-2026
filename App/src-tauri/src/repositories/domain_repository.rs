//! Persistent storage via SeaORM's SQLite connection and bound statements.
//! Table names and join names always come from the explicit domain whitelist.
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, TransactionTrait, Value as DbValue};
use serde_json::{Map, Value};

use crate::services::domain_service::{api_to_column_value, column_to_api, json_field, DomainResource};

#[derive(Clone)]
struct Column {
    name: String,
    sql_type: String,
    not_null: bool,
    default_value: bool,
    primary_key: bool,
}

fn quote(name: &str) -> String { format!("\"{}\"", name.replace('"', "\"\"")) }
fn stmt(sql: String, values: Vec<DbValue>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values)
}

async fn columns<C: ConnectionTrait>(conn: &C, table: &str) -> Result<Vec<Column>, String> {
    let query = format!("PRAGMA table_info({})", quote(table));
    let rows = conn.query_all_raw(Statement::from_string(DbBackend::Sqlite, query))
        .await.map_err(|e| e.to_string())?;
    rows.iter().map(|r| Ok(Column {
        name: r.try_get("", "name").map_err(|e| e.to_string())?,
        sql_type: r.try_get("", "type").map_err(|e| e.to_string())?,
        not_null: r.try_get::<i64>("", "notnull").map_err(|e| e.to_string())? != 0,
        default_value: r.try_get::<Option<String>>("", "dflt_value").map_err(|e| e.to_string())?.is_some(),
        primary_key: r.try_get::<i64>("", "pk").map_err(|e| e.to_string())? != 0,
    })).collect()
}

fn from_json(value: &Value, col: &Column) -> Result<DbValue, String> {
    if value.is_null() || (value == "" && (col.sql_type.to_uppercase().contains("INT") || col.sql_type.to_uppercase().contains("REAL"))) {
        return Ok(DbValue::String(None));
    }
    if json_field(&col.name) {
        return Ok(value.to_string().into());
    }
    Ok(match value {
        Value::Bool(b) => (*b).into(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() { i.into() }
            else { n.as_f64().ok_or("Invalid numeric value")?.into() }
        }
        Value::String(s) => s.clone().into(),
        Value::Array(_) | Value::Object(_) => return Err(format!("Unsupported complex value for {}", col.name)),
        Value::Null => DbValue::String(None),
    })
}

fn default_value(col: &Column) -> Option<Value> {
    if col.primary_key || !col.not_null || col.default_value { return None; }
    if col.name.ends_with("_id") || col.name.ends_with("_id_id") { return None; }
    if col.name == "datum" { return Some(Value::String(chrono::Utc::now().format("%Y-%m-%d").to_string())); }
    if col.name == "hochgeladen_am" || col.name == "zuletzt_geaendert_am" { return Some(Value::String(chrono_now_approx())); }
    let t = col.sql_type.to_uppercase();
    if t.contains("BOOL") || t.contains("INT") { Some(Value::from(0)) }
    else if t.contains("REAL") || t.contains("FLOAT") { Some(Value::from(0.0)) }
    else { Some(Value::String(String::new())) }
}
fn chrono_now_approx() -> String { chrono::Utc::now().to_rfc3339() }

async fn load<C: ConnectionTrait>(conn: &C, spec: DomainResource, cols: &[Column], filters: &[(String, Value)]) -> Result<Vec<Value>, String> {
    let expressions = cols.iter().flat_map(|c| [format!("'{}'", c.name.replace('\'', "''")), quote(&c.name)]).collect::<Vec<_>>();
    let mut sql = format!("SELECT json_object({}) AS payload FROM {}", expressions.join(","), quote(spec.table));
    let mut binds = Vec::new();
    for (i,(name,v)) in filters.iter().enumerate() {
        if !cols.iter().any(|c| c.name == *name) { return Err(format!("Invalid filter: {name}")); }
        sql.push_str(if i==0 {" WHERE "} else {" AND "});
        sql.push_str(&format!("{} = ?", quote(name)));
        binds.push(from_json(v, cols.iter().find(|c| c.name == *name).unwrap())?);
    }
    if cols.iter().any(|c|c.name == "sort_order") { sql.push_str(" ORDER BY sort_order, id"); }
    else { sql.push_str(" ORDER BY id"); }
    let rows = conn.query_all_raw(stmt(sql, binds)).await.map_err(|e| e.to_string())?;
    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        let raw: String = row.try_get("", "payload").map_err(|e| e.to_string())?;
        let Value::Object(map) = serde_json::from_str::<Value>(&raw).map_err(|e| e.to_string())? else { return Err("Expected JSON object".into()); };
        let mut out = Map::new();
        for (column, v) in map {
            let col = cols.iter().find(|c| c.name == column).ok_or("Missing column")?;
            let val = if json_field(&column) {
                match &v { Value::String(s) => serde_json::from_str(s).unwrap_or(Value::String(s.to_owned())), _ => v }
            } else if col.sql_type.to_uppercase().contains("BOOL") { v.as_i64().map(|n| Value::Bool(n != 0)).unwrap_or(v) }
            else { v };
            out.insert(column_to_api(&column), val);
        }
        if let Some(id) = out.get("id").and_then(Value::as_i64) {
            for rel in spec.relations {
                let q = format!("SELECT COALESCE(json_group_array({}), '[]') AS ids FROM {} WHERE {}=?", quote(rel.target_key), quote(rel.table), quote(rel.parent_key));
                let row = conn.query_one_raw(stmt(q, vec![id.into()])).await.map_err(|e| e.to_string())?
                    .ok_or("Missing relationship result")?;
                let value: String = row.try_get("", "ids").map_err(|e| e.to_string())?;
                out.insert(rel.property.into(), serde_json::from_str(&value).map_err(|e| e.to_string())?);
            }
        }
        result.push(Value::Object(out));
    }
    Ok(result)
}


// Django uses multi-table inheritance for its media subclasses.
async fn media_child_save<C: ConnectionTrait>(conn: &C, id: i64, payload: &Value) -> Result<(),String> {
    let Some(input) = payload.as_object() else { return Ok(()); };
    let media_type = input.get("media_type").and_then(Value::as_str).unwrap_or("").to_lowercase();
    let table = match media_type.as_str() {
        "image" => Some("bauwerksdiagnostik_imagemedia"),
        "document" => Some("bauwerksdiagnostik_documentmedia"),
        "messdatei" => Some("bauwerksdiagnostik_messdateimedia"),
        _ => None,
    };
    // When patching image annotations, the caller can omit media_type;
    // detect existing subclass instead.
    let table = if let Some(t) = table { Some(t) } else {
        let mut result = None;
        for t in ["bauwerksdiagnostik_imagemedia", "bauwerksdiagnostik_documentmedia", "bauwerksdiagnostik_messdateimedia"] {
            let sql = format!("SELECT 1 AS present FROM {} WHERE media_ptr_id=?", quote(t));
            if conn.query_one_raw(stmt(sql, vec![id.into()])).await.map_err(|e| e.to_string())?.is_some() {
                result = Some(t); break;
            }
        }
        result
    };
    let Some(table) = table else { return Ok(()); };
    let cols = columns(conn, table).await?;
    let mut names = vec!["media_ptr_id".to_string()];
    let mut values: Vec<DbValue> = vec![id.into()];
    for c in cols.iter().filter(|c| c.name != "media_ptr_id") {
        if let Some(v) = input.get(&c.name) {
            names.push(c.name.clone());
            values.push(from_json(v,c)?);
        } else if let Some(v) = default_value(c) {
            names.push(c.name.clone());
            values.push(from_json(&v,c)?);
        }
    }
    let sql = format!("INSERT OR IGNORE INTO {} ({}) VALUES ({})",quote(table),names.iter().map(|x|quote(x)).collect::<Vec<_>>().join(","),vec!["?";names.len()].join(","));
    conn.execute_raw(stmt(sql,values)).await.map_err(|e|e.to_string())?;
    let mut updates=Vec::new();let mut binds=Vec::new();
    for c in cols.iter().filter(|c| c.name != "media_ptr_id") {
        if let Some(v) = input.get(&c.name) {
            updates.push(format!("{}=?", quote(&c.name)));
            binds.push(from_json(v,c)?);
        }
    }
    if !updates.is_empty() {
        let sql = format!("UPDATE {} SET {} WHERE media_ptr_id=?",quote(table),updates.join(","));
        binds.push(id.into());
        conn.execute_raw(stmt(sql,binds)).await.map_err(|e|e.to_string())?;
    }
    Ok(())
}
async fn media_child_load<C: ConnectionTrait>(conn:&C, id:i64, row:&mut Value) -> Result<(),String> {
    let Some(map) = row.as_object_mut() else { return Ok(()); };
    for (table,kind) in [
        ("bauwerksdiagnostik_imagemedia","image"),
        ("bauwerksdiagnostik_documentmedia","document"),
        ("bauwerksdiagnostik_messdateimedia","messdatei")
    ] {
        let cols=columns(conn,table).await?;
        let keys=cols.iter().flat_map(|c|[format!("'{}'",c.name),quote(&c.name)]).collect::<Vec<_>>().join(",");
        let sql=format!("SELECT json_object({keys}) AS payload FROM {} WHERE media_ptr_id=?",quote(table));
        if let Some(record)=conn.query_one_raw(stmt(sql,vec![id.into()])).await.map_err(|e|e.to_string())? {
            let raw:String=record.try_get("","payload").map_err(|e|e.to_string())?;
            let val:Value=serde_json::from_str(&raw).map_err(|e|e.to_string())?;
            if let Some(values)=val.as_object() {
                for (k,v) in values {
                    if k != "media_ptr_id" {
                        let v=if json_field(k) { v.as_str().and_then(|text|serde_json::from_str(text).ok()).unwrap_or_else(||v.clone()) } else {v.clone()};
                        map.insert(k.clone(),v);
                    }
                }
            }
            map.insert("media_type".into(),Value::String(kind.into()));
            return Ok(());
        }
    }
    map.insert("media_type".into(),Value::String("media".into()));
    Ok(())
}
pub async fn list(db: &DatabaseConnection, spec: DomainResource, filters: Vec<(String, Value)>) -> Result<Vec<Value>, String> {
    let cols = columns(db, spec.table).await?;
    let mut rows = load(db, spec, &cols, &filters).await?;
    if spec.table == "bauwerksdiagnostik_media" {
        for row in &mut rows {
            if let Some(id) = row.get("id").and_then(Value::as_i64) { media_child_load(db, id, row).await?; }
        }
    }
    Ok(rows)
}

pub async fn get(db: &DatabaseConnection, spec: DomainResource, id: i64) -> Result<Value, String> {
    let cols = columns(db, spec.table).await?;
    let mut row = load(db, spec, &cols, &[("id".into(), Value::from(id))]).await?.into_iter().next()
        .ok_or_else(|| format!("Record {id} not found"))?;
    if spec.table == "bauwerksdiagnostik_media" { media_child_load(db, id, &mut row).await?; }
    Ok(row)
}

async fn save<C: ConnectionTrait>(conn: &C, spec: DomainResource, payload: &Value, update_id: Option<i64>, import: bool, with_relations: bool) -> Result<i64, String> {
    let input = payload.as_object().ok_or("Expected object payload")?;
    let cols = columns(conn, spec.table).await?;
    let mut fields = Vec::<String>::new();
    let mut values = Vec::<DbValue>::new();
    for col in &cols {
        if col.primary_key && (update_id.is_some() || !import) { continue; }
        let actual = api_to_column_value(input, &col.name);
        let supplied = actual.cloned().or_else(|| if update_id.is_none() && !import { default_value(col) } else { None });
        if let Some(value) = supplied {
            fields.push(col.name.clone());
            values.push(from_json(&value, col)?);
        }
    }
    let row_id = if let Some(id) = update_id {
        if fields.is_empty() { id } else {
            let changes = fields.iter().map(|f| format!("{}=?", quote(f))).collect::<Vec<_>>();
            let query = format!("UPDATE {} SET {} WHERE id=?", quote(spec.table), changes.join(","));
            values.push(id.into());
            let result = conn.execute_raw(stmt(query, values)).await.map_err(|e| e.to_string())?;
            if result.rows_affected() == 0 { return Err(format!("Record {id} not found")); }
            id
        }
    } else {
        if import && !input.get("id").and_then(Value::as_i64).is_some() { return Err("Import requires an id".into()); }
        let sql = if fields.is_empty() { format!("INSERT INTO {} DEFAULT VALUES", quote(spec.table)) }
        else {
            format!("INSERT {}INTO {} ({}) VALUES ({})", if import {"OR IGNORE "} else {""}, quote(spec.table), fields.iter().map(|f|quote(f)).collect::<Vec<_>>().join(","), vec!["?"; fields.len()].join(","))
        };
        let res = conn.execute_raw(stmt(sql, values)).await.map_err(|e| e.to_string())?;
        if import { input.get("id").and_then(Value::as_i64).unwrap() }
        else { res.last_insert_id() as i64 }
    };
    if with_relations { set_links(conn, spec, row_id, input, true).await?; }
    Ok(row_id)
}

async fn set_links<C: ConnectionTrait>(conn: &C, spec: DomainResource, id: i64, map: &Map<String, Value>, replace: bool) -> Result<(), String> {
    for rel in spec.relations {
        if let Some(value) = map.get(rel.property) {
            let ids = value.as_array().ok_or_else(|| format!("{} must be an array",rel.property))?;
            let del = format!("DELETE FROM {} WHERE {}=?", quote(rel.table), quote(rel.parent_key));
            if replace { conn.execute_raw(stmt(del, vec![id.into()])).await.map_err(|e| e.to_string())?; }
            for value in ids {
                let other = value.as_i64().ok_or("Expected numeric foreign key")?;
                let insert = format!("INSERT OR IGNORE INTO {} ({}, {}) VALUES (?, ?)", quote(rel.table), quote(rel.parent_key), quote(rel.target_key));
                conn.execute_raw(stmt(insert, vec![id.into(), other.into()])).await.map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

pub async fn create(db: &DatabaseConnection, spec: DomainResource, payload: Value) -> Result<Value,String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    let id = save(&txn, spec, &payload, None, false, true).await?;
    if spec.table == "bauwerksdiagnostik_media" { media_child_save(&txn, id, &payload).await?; }
    txn.commit().await.map_err(|e| e.to_string())?;
    get(db, spec, id).await
}
pub async fn update(db: &DatabaseConnection, spec: DomainResource, id: i64, payload: Value) -> Result<Value,String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    let id = save(&txn, spec, &payload, Some(id), false, true).await?;
    if spec.table == "bauwerksdiagnostik_media" { media_child_save(&txn, id, &payload).await?; }
    txn.commit().await.map_err(|e| e.to_string())?;
    get(db, spec, id).await
}
pub async fn delete(db: &DatabaseConnection, spec: DomainResource, id: i64) -> Result<(),String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    for rel in spec.relations {
        let sql = format!("DELETE FROM {} WHERE {}=?", quote(rel.table), quote(rel.parent_key));
        txn.execute_raw(stmt(sql, vec![id.into()])).await.map_err(|e| e.to_string())?;
    }
    if spec.table == "bauwerksdiagnostik_media" {
        // Model inheritance and many-to-many links must be removed before Media.
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
            let sql = format!("DELETE FROM {table} WHERE media_id=?");
            txn.execute_raw(stmt(sql, vec![id.into()])).await.map_err(|e|e.to_string())?;
        }
        for table in [
            "bauwerksdiagnostik_chloridanalyse",
            "bauwerksdiagnostik_druckfestigkeitspruefung",
            "bauwerksdiagnostik_kernbohrung",
            "bauwerksdiagnostik_rissbreitenmessung",
            "bauwerksdiagnostik_rueckprallhammer",
        ] {
            let sql = format!("UPDATE {table} SET titelBild_id=NULL WHERE titelBild_id=?");
            txn.execute_raw(stmt(sql,vec![id.into()])).await.map_err(|e|e.to_string())?;
        }
        txn.execute_raw(stmt("UPDATE bauwerksdiagnostik_druckfestigkeitspruefung SET pruefbericht_id=NULL WHERE pruefbericht_id=?".into(),vec![id.into()])).await.map_err(|e|e.to_string())?;
        for table in ["bauwerksdiagnostik_imagemedia","bauwerksdiagnostik_documentmedia","bauwerksdiagnostik_messdateimedia"] {
            let sql=format!("DELETE FROM {table} WHERE media_ptr_id=?");
            txn.execute_raw(stmt(sql,vec![id.into()])).await.map_err(|e|e.to_string())?;
        }
    }
    let sql = format!("DELETE FROM {} WHERE id=?", quote(spec.table));
    let result = txn.execute_raw(stmt(sql, vec![id.into()])).await.map_err(|e| e.to_string())?;
    if result.rows_affected()==0 { return Err(format!("Record {id} not found")); }
    txn.commit().await.map_err(|e| e.to_string())
}

// One-way, explicit initial import. Does not update existing local records.
pub async fn import(db: &DatabaseConnection, spec: DomainResource, records: Vec<Value>) -> Result<usize,String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    for record in &records {
        let id = save(&txn, spec, record, None, true, false).await?;
        if spec.table == "bauwerksdiagnostik_media" { media_child_save(&txn, id, record).await?; }
    }
    txn.commit().await.map_err(|e| e.to_string())?;
    Ok(records.len())
}
pub async fn import_links(db: &DatabaseConnection, spec: DomainResource, records: Vec<Value>) -> Result<(),String> {
    let txn = db.begin().await.map_err(|e| e.to_string())?;
    for record in records {
        let map = record.as_object().ok_or("Expected record")?;
        let id = map.get("id").and_then(Value::as_i64).ok_or("Relation import requires id")?;
        set_links(&txn, spec, id, map, false).await?;
    }
    txn.commit().await.map_err(|e| e.to_string())
}
