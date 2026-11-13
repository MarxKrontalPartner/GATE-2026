//! SeaORM entity for SQLite table `bauwerksdiagnostik_untersuchungsstelle`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "bauwerksdiagnostik_untersuchungsstelle")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub bemerkung: String,
    #[sea_orm(column_name = "lagenbezugHoehe")]
    pub lagenbezug_hoehe: Option<String>,
    pub hoehe: Option<f64>,
    #[sea_orm(column_name = "lagenbezugAbstand1")]
    pub lagenbezug_abstand1: Option<String>,
    pub abstand1: Option<f64>,
    #[sea_orm(column_name = "lagenbezugAbstand2")]
    pub lagenbezug_abstand2: Option<String>,
    pub abstand2: Option<f64>,
    pub bauwerk_id: i64,
    pub projekt_id: Option<i64>,
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
