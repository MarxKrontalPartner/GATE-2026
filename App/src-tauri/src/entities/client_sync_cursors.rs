//! SeaORM entity for SQLite table `client_sync_cursors`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_sync_cursors")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub dataset_key: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub resource_key: String,
    pub cursor: Option<String>,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
