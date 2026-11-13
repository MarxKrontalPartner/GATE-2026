//! SeaORM entity for SQLite table `bauwerksdiagnostik_rueckprallhammer`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "bauwerksdiagnostik_rueckprallhammer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub datum: Date,
    pub zeit: Option<Time>,
    pub bemerkung: String,
    pub lufttemperatur: Option<f64>,
    pub luftfeuchte: Option<f64>,
    pub abgeschlossen: Option<bool>,
    pub sort_order: i32,
    pub hammertyp: String,
    pub schlagrichtung: String,
    #[sea_orm(column_name = "oberflaecheGeschliffen")]
    pub oberflaeche_geschliffen: Option<bool>,
    #[sea_orm(column_name = "kalibriertAm")]
    pub kalibriert_am: Option<Date>,
    pub messreihe: Option<Json>,
    #[sea_orm(column_name = "medianRueckprallwert")]
    pub median_rueckprallwert: Option<f64>,
    pub bauwerk_id: Option<i64>,
    pub projekt_id: Option<i64>,
    #[sea_orm(column_name = "titelBild_id")]
    pub titel_bild_id: Option<i64>,
    pub untersuchungsstelle_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bauwerksdiagnostik_untersuchungsstelle::Entity",
        from = "Column::UntersuchungsstelleId",
        to = "super::bauwerksdiagnostik_untersuchungsstelle::Column::Id"
    )]
    Untersuchungsstelle,
    #[sea_orm(
        belongs_to = "super::bauwerksdiagnostik_media::Entity",
        from = "Column::TitelBildId",
        to = "super::bauwerksdiagnostik_media::Column::Id"
    )]
    TitelBild,
    #[sea_orm(
        belongs_to = "super::assetverwaltung_projekt::Entity",
        from = "Column::ProjektId",
        to = "super::assetverwaltung_projekt::Column::Id"
    )]
    Projekt,
    #[sea_orm(
        belongs_to = "super::assetverwaltung_bauwerk::Entity",
        from = "Column::BauwerkId",
        to = "super::assetverwaltung_bauwerk::Column::Id"
    )]
    Bauwerk,
}

impl Related<super::bauwerksdiagnostik_untersuchungsstelle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Untersuchungsstelle.def()
    }
}

impl Related<super::bauwerksdiagnostik_media::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TitelBild.def()
    }
}

impl Related<super::assetverwaltung_projekt::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projekt.def()
    }
}

impl Related<super::assetverwaltung_bauwerk::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bauwerk.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
