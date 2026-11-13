//! SeaORM entity for SQLite table `assetverwaltung_bauwerk`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "assetverwaltung_bauwerk")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub bezeichnung: String,
    pub archived: bool,
    pub beschreibung: String,
    pub strasse: String,
    pub bauweise: String,
    pub bauwerksnummer: String,
    pub bauwerkstyp: Option<String>,
    pub bemerkung: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
