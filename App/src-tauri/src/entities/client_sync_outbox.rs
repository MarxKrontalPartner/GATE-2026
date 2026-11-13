//! SeaORM entity for SQLite table `client_sync_outbox`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_sync_outbox")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub operation_id: String,
    pub entity_table: String,
    pub local_pk: String,
    pub operation: String,
    pub base_server_revision: Option<String>,
    pub payload_json: Option<Json>,
    pub status: String,
    pub attempt_count: i32,
    pub last_error: Option<String>,
    pub created_at: String,
    pub last_attempt_at: Option<String>,
    pub acknowledged_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
