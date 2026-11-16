//! SeaORM mapping for dashboard-only SQLite metadata `client_dashboard_recent_bauwerk`.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_dashboard_recent_bauwerk")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub bauwerk_id: i64,
    pub last_opened_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
