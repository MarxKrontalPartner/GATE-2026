//! SeaORM entity for SQLite table `bauwerksdiagnostik_rissbreitenmessung_bearbeiter`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "bauwerksdiagnostik_rissbreitenmessung_bearbeiter")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub rissbreitenmessung_id: i64,
    pub person_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assetverwaltung_person::Entity",
        from = "Column::PersonId",
        to = "super::assetverwaltung_person::Column::Id"
    )]
    Person,
    #[sea_orm(
        belongs_to = "super::bauwerksdiagnostik_rissbreitenmessung::Entity",
        from = "Column::RissbreitenmessungId",
        to = "super::bauwerksdiagnostik_rissbreitenmessung::Column::Id"
    )]
    Rissbreitenmessung,
}

impl Related<super::assetverwaltung_person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl Related<super::bauwerksdiagnostik_rissbreitenmessung::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rissbreitenmessung.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
