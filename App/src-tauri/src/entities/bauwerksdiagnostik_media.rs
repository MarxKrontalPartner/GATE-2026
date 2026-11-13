//! SeaORM entity for SQLite table `bauwerksdiagnostik_media`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "bauwerksdiagnostik_media")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub beschreibung: String,
    pub local_id: Option<i32>,
    pub datei: String,
    pub hochgeladen_von: String,
    pub hochgeladen_am: DateTime,
    pub zuletzt_geaendert_am: DateTime,
    pub model_name: String,
    pub model_pk: i32,
    pub bauwerk_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assetverwaltung_bauwerk::Entity",
        from = "Column::BauwerkId",
        to = "super::assetverwaltung_bauwerk::Column::Id"
    )]
    Bauwerk,
}

impl Related<super::assetverwaltung_bauwerk::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bauwerk.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
