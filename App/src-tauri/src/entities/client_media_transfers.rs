//! SeaORM entity for SQLite table `client_media_transfers`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_media_transfers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub transfer_id: String,
    pub media_local_pk: Option<String>,
    pub local_file_path: String,
    pub content_sha256: Option<String>,
    pub mime_type: Option<String>,
    pub byte_size: Option<i32>,
    pub remote_file_key: Option<String>,
    pub direction: String,
    pub status: String,
    pub retries: i32,
    pub last_error: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
