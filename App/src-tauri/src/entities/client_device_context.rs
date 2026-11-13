//! SeaORM entity for SQLite table `client_device_context`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_device_context")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub device_id: String,
    pub registered_at: String,
    pub server_registered_at: Option<String>,
    pub last_successful_sync_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
