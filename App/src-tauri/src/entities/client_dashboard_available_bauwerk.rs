//! SeaORM mapping for dashboard-only SQLite metadata `client_dashboard_available_bauwerk`.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_dashboard_available_bauwerk")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub bauwerk_id: i64,
    pub bezeichnung: String,
    pub bauwerksnummer: Option<String>,
    pub archived: bool,
    pub projekte_json: String,
    pub last_seen_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
