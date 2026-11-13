//! SeaORM entity for SQLite table `assetverwaltung_bauwerk_projekt`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "assetverwaltung_bauwerk_projekt")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub bauwerk_id: i64,
    pub projekt_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assetverwaltung_projekt::Entity",
        from = "Column::ProjektId",
        to = "super::assetverwaltung_projekt::Column::Id"
    )]
    Projekt,
    #[sea_orm(
        belongs_to = "super::assetverwaltung_bauwerk::Entity",
        from = "Column::BauwerkId",
        to = "super::assetverwaltung_bauwerk::Column::Id"
    )]
    Bauwerk,
}

impl Related<super::assetverwaltung_projekt::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projekt.def()
    }
}

impl Related<super::assetverwaltung_bauwerk::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bauwerk.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
