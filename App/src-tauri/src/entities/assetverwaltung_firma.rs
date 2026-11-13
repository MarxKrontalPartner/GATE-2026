//! SeaORM entity for SQLite table `assetverwaltung_firma`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "assetverwaltung_firma")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub bezeichnung: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
