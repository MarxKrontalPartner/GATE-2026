//! SeaORM entity for SQLite table `assetverwaltung_projekt`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "assetverwaltung_projekt")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub projektnummer: String,
    pub name: String,
    pub bezeichnung: String,
    pub start: Option<Date>,
    pub ende: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
