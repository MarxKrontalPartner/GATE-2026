//! SeaORM entity for SQLite table `client_sync_tombstones`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_sync_tombstones")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub entity_table: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub local_pk: String,
    pub client_uuid: String,
    pub server_pk: Option<String>,
    pub base_server_revision: Option<String>,
    pub deleted_at: String,
    pub acknowledged_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
