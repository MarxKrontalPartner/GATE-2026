//! SeaORM mapping for dashboard-only SQLite metadata `client_dashboard_bauwerk_baseline`.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_dashboard_bauwerk_baseline")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub bauwerk_id: i64,
    pub server_snapshot_json: String,
    pub downloaded_at: String,
    pub last_checked_at: Option<String>,
    pub server_update_available: bool,
    pub seen_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
