//! SeaORM entity for SQLite table `bauwerksdiagnostik_bohrkern_fotos`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "bauwerksdiagnostik_bohrkern_fotos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub bohrkern_id: i64,
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
        belongs_to = "super::bauwerksdiagnostik_bohrkern::Entity",
        from = "Column::BohrkernId",
        to = "super::bauwerksdiagnostik_bohrkern::Column::Id"
    )]
    Bohrkern,
}

impl Related<super::bauwerksdiagnostik_media::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Media.def()
    }
}

impl Related<super::bauwerksdiagnostik_bohrkern::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bohrkern.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
