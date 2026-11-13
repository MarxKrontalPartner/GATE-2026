//! SeaORM entity for SQLite table `client_record_state`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_record_state")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub entity_table: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub local_pk: String,
    pub client_uuid: String,
    pub server_pk: Option<String>,
    pub base_server_revision: Option<String>,
    pub local_revision: i32,
    pub sync_status: String,
    pub updated_at: String,
    pub last_synced_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
