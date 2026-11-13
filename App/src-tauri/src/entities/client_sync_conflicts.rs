//! SeaORM entity for SQLite table `client_sync_conflicts`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_sync_conflicts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub conflict_id: String,
    pub entity_table: String,
    pub local_pk: String,
    pub operation_id: Option<String>,
    pub reason: String,
    pub base_snapshot_json: Option<Json>,
    pub local_snapshot_json: Json,
    pub server_snapshot_json: Option<Json>,
    pub server_revision: Option<String>,
    pub status: String,
    pub resolution_json: Option<Json>,
    pub created_at: String,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::client_sync_outbox::Entity",
        from = "Column::OperationId",
        to = "super::client_sync_outbox::Column::OperationId"
    )]
    Operation,
}

impl Related<super::client_sync_outbox::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Operation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
