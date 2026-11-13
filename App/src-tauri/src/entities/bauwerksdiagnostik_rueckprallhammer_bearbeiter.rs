//! SeaORM entity for SQLite table `bauwerksdiagnostik_rueckprallhammer_bearbeiter`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "bauwerksdiagnostik_rueckprallhammer_bearbeiter")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub rueckprallhammer_id: i64,
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
        belongs_to = "super::bauwerksdiagnostik_rueckprallhammer::Entity",
        from = "Column::RueckprallhammerId",
        to = "super::bauwerksdiagnostik_rueckprallhammer::Column::Id"
    )]
    Rueckprallhammer,
}

impl Related<super::assetverwaltung_person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl Related<super::bauwerksdiagnostik_rueckprallhammer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rueckprallhammer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
