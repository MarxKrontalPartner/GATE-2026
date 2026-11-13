//! SeaORM entity for SQLite table `client_cached_projects`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_cached_projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub project_id: i64,
    pub download_status: String,
    pub last_full_download_at: Option<String>,
    pub last_sync_at: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assetverwaltung_projekt::Entity",
        from = "Column::ProjectId",
        to = "super::assetverwaltung_projekt::Column::Id"
    )]
    Project,
}

impl Related<super::assetverwaltung_projekt::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
