//! SeaORM entity for SQLite table `bauwerksdiagnostik_documentmedia`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "bauwerksdiagnostik_documentmedia")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub media_ptr_id: i64,
    pub file_format: String,
    pub preview: String,
    pub preview_image: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bauwerksdiagnostik_media::Entity",
        from = "Column::MediaPtrId",
        to = "super::bauwerksdiagnostik_media::Column::Id"
    )]
    MediaPtr,
}

impl Related<super::bauwerksdiagnostik_media::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaPtr.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
