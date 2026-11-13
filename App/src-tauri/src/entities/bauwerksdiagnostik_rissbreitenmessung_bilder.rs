//! SeaORM entity for SQLite table `bauwerksdiagnostik_rissbreitenmessung_bilder`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "bauwerksdiagnostik_rissbreitenmessung_bilder")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub rissbreitenmessung_id: i64,
    pub media_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bauwerksdiagnostik_media::Entity",
        from = "Column::MediaId",
        to = "super::bauwerksdiagnostik_media::Column::Id"
    )]
    Media,
    #[sea_orm(
        belongs_to = "super::bauwerksdiagnostik_rissbreitenmessung::Entity",
        from = "Column::RissbreitenmessungId",
        to = "super::bauwerksdiagnostik_rissbreitenmessung::Column::Id"
    )]
    Rissbreitenmessung,
}

impl Related<super::bauwerksdiagnostik_media::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Media.def()
    }
}

impl Related<super::bauwerksdiagnostik_rissbreitenmessung::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rissbreitenmessung.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
