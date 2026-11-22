//! Whitelisted domain resources and relationship definitions.
//! assetverwaltung provides references; diagnostic forms are our editing focus.

#[derive(Clone, Copy)]
pub struct Relation {
    pub property: &'static str,
    pub table: &'static str,
    pub parent_key: &'static str,
    pub target_key: &'static str,
}

#[derive(Clone, Copy)]
pub struct DomainResource {
    pub table: &'static str,
    pub relations: &'static [Relation],
}

const fn rel(property: &'static str, table: &'static str, parent: &'static str, target: &'static str) -> Relation {
    Relation { property, table, parent_key: parent, target_key: target }
}
const BAUWERK: &[Relation] = &[rel("projekt", "assetverwaltung_bauwerk_projekt", "bauwerk_id", "projekt_id")];
const STELLE: &[Relation] = &[rel("media", "bauwerksdiagnostik_untersuchungsstelle_media", "untersuchungsstelle_id", "media_id")];
const BOHRKERN: &[Relation] = &[
    rel("fotos", "bauwerksdiagnostik_bohrkern_fotos", "bohrkern_id", "media_id"),
    rel("material", "bauwerksdiagnostik_bohrkern_material", "bohrkern_id", "material_id"),
];
const PROBE: &[Relation] = &[
    rel("foto", "bauwerksdiagnostik_probe_foto", "probe_id", "media_id"),
    rel("material", "bauwerksdiagnostik_probe_material", "probe_id", "material_id"),
];
const RISS: &[Relation] = &[
    rel("bearbeiter", "bauwerksdiagnostik_rissbreitenmessung_bearbeiter", "rissbreitenmessung_id", "person_id"),
    rel("bilder", "bauwerksdiagnostik_rissbreitenmessung_bilder", "rissbreitenmessung_id", "media_id"),
];
const RUECK: &[Relation] = &[
    rel("bearbeiter", "bauwerksdiagnostik_rueckprallhammer_bearbeiter", "rueckprallhammer_id", "person_id"),
    rel("bilder", "bauwerksdiagnostik_rueckprallhammer_bilder", "rueckprallhammer_id", "media_id"),
    rel("messprotokolle", "bauwerksdiagnostik_rueckprallhammer_messprotokolle", "rueckprallhammer_id", "media_id"),
];
const KERN: &[Relation] = &[
    rel("bearbeiter", "bauwerksdiagnostik_kernbohrung_bearbeiter", "kernbohrung_id", "person_id"),
    rel("bilder", "bauwerksdiagnostik_kernbohrung_bilder", "kernbohrung_id", "media_id"),
];
const CHLOR: &[Relation] = &[
    rel("bearbeiter", "bauwerksdiagnostik_chloridanalyse_bearbeiter", "chloridanalyse_id", "person_id"),
    rel("bilder", "bauwerksdiagnostik_chloridanalyse_bilder", "chloridanalyse_id", "media_id"),
    rel("pruefberichte", "bauwerksdiagnostik_chloridanalyse_pruefberichte", "chloridanalyse_id", "media_id"),
];
const DRUCK: &[Relation] = &[
    rel("bearbeiter", "bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter", "druckfestigkeitspruefung_id", "person_id"),
    rel("bilder", "bauwerksdiagnostik_druckfestigkeitspruefung_bilder", "druckfestigkeitspruefung_id", "media_id"),
];

pub fn resource(name: &str) -> Result<DomainResource, String> {
    let lowered = name.trim().to_lowercase();
    let (table, relations): (&'static str, &'static [Relation]) = match lowered.as_str() {
        "bauwerk" => ("assetverwaltung_bauwerk", BAUWERK),
        "projekt" => ("assetverwaltung_projekt", &[]),
        "person" | "bearbeiter" => ("assetverwaltung_person", &[]),
        "firma" => ("assetverwaltung_firma", &[]),
        "untersuchungsstelle" => ("bauwerksdiagnostik_untersuchungsstelle", STELLE),
        "bohrkern" => ("bauwerksdiagnostik_bohrkern", BOHRKERN),
        "probe" => ("bauwerksdiagnostik_probe", PROBE),
        "material" => ("bauwerksdiagnostik_material", &[]),
        "media" => ("bauwerksdiagnostik_media", &[]),
        "rissbreitenmessung" => ("bauwerksdiagnostik_rissbreitenmessung", RISS),
        "rueckprallhammer" => ("bauwerksdiagnostik_rueckprallhammer", RUECK),
        "kernbohrung" => ("bauwerksdiagnostik_kernbohrung", KERN),
        "chloridanalyse" => ("bauwerksdiagnostik_chloridanalyse", CHLOR),
        "druckfestigkeitspruefung" => ("bauwerksdiagnostik_druckfestigkeitspruefung", DRUCK),
        _ => return Err(format!("Unsupported local resource: {name}")),
    };
    Ok(DomainResource { table, relations })
}

/// Field-level sync configuration for one domain table.
/// `fields` must match the server model's `EDITABLE` list (API names, no M2M).
#[derive(Clone, Copy)]
pub struct SyncSpec {
    /// Path of the Django sync action, relative to the API base URL.
    pub endpoint: &'static str,
    /// Path for creating records (POST, accepts a list), relative to the API base URL.
    pub create_endpoint: &'static str,
    pub fields: &'static [&'static str],
}

// Sync field lists: must match the server models' EDITABLE lists (API names,
// no M2M fields, no media links). Common fields first, then the form's own.
const RUECK_SYNC_FIELDS: &[&str] = &[
    "hammertyp", "schlagrichtung", "oberflaecheGeschliffen", "kalibriertAm",
    "messreihe", "medianRueckprallwert", "datum", "zeit", "bemerkung",
    "lufttemperatur", "luftfeuchte", "abgeschlossen",
    "untersuchungsstelle", "projekt", "bauwerk",
];
const KERN_SYNC_FIELDS: &[&str] = &[
    "datum", "zeit", "bemerkung", "lufttemperatur", "luftfeuchte", "abgeschlossen",
    "untersuchungsstelle", "projekt", "bauwerk",
    "durchmesser", "bohrlaenge", "neigungswinkel", "kuehlung", "verschlussmaterial",
    "bohrkern", "schichtaufbau", "bewehrungAngeschnitten", "kernVollstaendig",
];
const RISS_SYNC_FIELDS: &[&str] = &[
    "datum", "zeit", "bemerkung", "lufttemperatur", "luftfeuchte", "abgeschlossen",
    "untersuchungsstelle", "projekt", "bauwerk",
    "messmittel", "risse", "maxRissbreite", "monitoringEmpfohlen",
];
const CHLOR_SYNC_FIELDS: &[&str] = &[
    "datum", "zeit", "bemerkung", "lufttemperatur", "luftfeuchte", "abgeschlossen",
    "untersuchungsstelle", "projekt", "bauwerk",
    "probe", "pruefstelle", "analyseverfahren", "chloridgehaltZement",
    "zementgehaltAnnahme", "grenzwertUeberschritten",
];
const DRUCK_SYNC_FIELDS: &[&str] = &[
    "datum", "zeit", "bemerkung", "lufttemperatur", "luftfeuchte", "abgeschlossen",
    "untersuchungsstelle", "projekt", "bauwerk",
    "probe", "pruefalter", "lagerung", "bruchlast", "druckfestigkeit", "bruchbild",
];

/// All tables that are synchronized field by field (offline edit and create).
pub const SYNCED_TABLES: &[&str] = &[
    "bauwerksdiagnostik_rueckprallhammer",
    "bauwerksdiagnostik_kernbohrung",
    "bauwerksdiagnostik_rissbreitenmessung",
    "bauwerksdiagnostik_chloridanalyse",
    "bauwerksdiagnostik_druckfestigkeitspruefung",
];

/// Tables whose local edits are recorded in the outbox and sent as patches.
pub fn sync_spec(table: &str) -> Option<SyncSpec> {
    let (endpoint, create_endpoint, fields) = match table {
        "bauwerksdiagnostik_rueckprallhammer" =>
            ("bwd/RueckprallHammer/sync/", "bwd/RueckprallHammer/", RUECK_SYNC_FIELDS),
        "bauwerksdiagnostik_kernbohrung" =>
            ("bwd/Kernbohrung/sync/", "bwd/Kernbohrung/", KERN_SYNC_FIELDS),
        "bauwerksdiagnostik_rissbreitenmessung" =>
            ("bwd/Rissbreitenmessung/sync/", "bwd/Rissbreitenmessung/", RISS_SYNC_FIELDS),
        "bauwerksdiagnostik_chloridanalyse" =>
            ("bwd/Chloridanalyse/sync/", "bwd/Chloridanalyse/", CHLOR_SYNC_FIELDS),
        "bauwerksdiagnostik_druckfestigkeitspruefung" =>
            ("bwd/Druckfestigkeitspruefung/sync/", "bwd/Druckfestigkeitspruefung/", DRUCK_SYNC_FIELDS),
        _ => return None,
    };
    Some(SyncSpec { endpoint, create_endpoint, fields })
}

pub fn json_field(name: &str) -> bool {
    matches!(name, "risse" | "messreihe" | "schichtaufbau" | "historie" | "drawing" | "exif" | "measurement_data")
}

pub fn column_to_api(name: &str) -> String {
    name.strip_suffix("_id").unwrap_or(name).to_owned()
}

pub fn api_to_column_value<'a>(payload: &'a serde_json::Map<String, serde_json::Value>, col: &str) -> Option<&'a serde_json::Value> {
    payload.get(col).or_else(|| col.strip_suffix("_id").and_then(|key| payload.get(key)))
}

/// Business rules for existing diagnostic forms, separate from IPC and SQL.
/// These are local rules only: server authorization and cross-device checks
/// are deliberately outside the scope of this phase.
pub async fn prepare_for_save(
    db: &sea_orm::DatabaseConnection,
    spec: DomainResource,
    payload: serde_json::Value,
    creating: bool,
) -> Result<serde_json::Value, String> {
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
    use serde_json::Value;
    use crate::entities;
    let mut data = payload.as_object().ok_or("Expected object")?.clone();
    let diagnostic = matches!(spec.table,
        "bauwerksdiagnostik_rissbreitenmessung" |
        "bauwerksdiagnostik_rueckprallhammer" |
        "bauwerksdiagnostik_kernbohrung" |
        "bauwerksdiagnostik_chloridanalyse" |
        "bauwerksdiagnostik_druckfestigkeitspruefung"
    );
    if creating && diagnostic {
        for key in ["bauwerk", "projekt", "untersuchungsstelle"] {
            if !data.get(key).and_then(Value::as_i64).is_some_and(|v| v > 0) {
                return Err(format!("New diagnostic record requires {key}"));
            }
        }
        let order = data.get("sort_order").and_then(Value::as_i64).unwrap_or(0);
        if order <= 0 {
            let stelle_id = data["untersuchungsstelle"].as_i64().unwrap();
            let projekt_id = data["projekt"].as_i64().unwrap();
            // SeaORM finders use the entity fields. There is no SELECT string
            // to maintain when the database designer adds a new column.
            macro_rules! latest_order {
                ($name:ident) => {{
                    entities::$name::Entity::find()
                        .filter(entities::$name::Column::UntersuchungsstelleId.eq(stelle_id))
                        .filter(entities::$name::Column::ProjektId.eq(projekt_id))
                        .order_by_desc(entities::$name::Column::SortOrder)
                        .one(db)
                        .await
                        .map_err(|e| e.to_string())?
                        .map(|row| i64::from(row.sort_order))
                        .unwrap_or(0)
                }};
            }
            let max_order = [
                latest_order!(bauwerksdiagnostik_rissbreitenmessung),
                latest_order!(bauwerksdiagnostik_rueckprallhammer),
                latest_order!(bauwerksdiagnostik_kernbohrung),
                latest_order!(bauwerksdiagnostik_chloridanalyse),
                latest_order!(bauwerksdiagnostik_druckfestigkeitspruefung),
            ].into_iter().max().unwrap_or(0);
            data.insert("sort_order".into(), Value::from(max_order + 1));
        }
    }
    if creating && spec.table == "bauwerksdiagnostik_untersuchungsstelle" {
        if data.get("name").and_then(Value::as_str).unwrap_or("").trim().is_empty() {
            return Err("Untersuchungsstelle requires a name".into());
        }
        if !data.get("bauwerk").and_then(Value::as_i64).is_some_and(|v|v>0) {
            return Err("Untersuchungsstelle requires a Bauwerk".into());
        }
    }
    if creating && spec.table == "bauwerksdiagnostik_material" {
        if data.get("name").and_then(Value::as_str).unwrap_or("").trim().is_empty() {
            return Err("Material requires a name".into());
        }
    }
    Ok(Value::Object(data))
}
