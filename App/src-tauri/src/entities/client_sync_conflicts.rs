//! SeaORM entity for SQLite table `client_sync_conflicts` (migration 0008).
//! One row per field the server reported as a conflict during a sync.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_sync_conflicts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub conflict_id: String,
    pub entity_table: String,
    pub local_pk: String,
    pub column_name: String,
    pub server_value: Json,
    pub new_value: Json,
    pub server_revision: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
