//! SeaORM entity for SQLite table `bauwerksdiagnostik_probe`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "bauwerksdiagnostik_probe")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub probenbezeichnung: String,
    pub bemerkung: Option<String>,
    pub probendurchmesser: Option<f64>,
    pub probenlaenge: Option<f64>,
    #[sea_orm(column_name = "entnahmetiefeVon")]
    pub entnahmetiefe_von: Option<f64>,
    #[sea_orm(column_name = "entnahmetiefeBis")]
    pub entnahmetiefe_bis: Option<f64>,
    pub bauwerk_id_id: Option<i64>,
    pub bohrkern_id: Option<i64>,
    pub projekt_id_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assetverwaltung_projekt::Entity",
        from = "Column::ProjektIdId",
        to = "super::assetverwaltung_projekt::Column::Id"
    )]
    ProjektId,
    #[sea_orm(
        belongs_to = "super::bauwerksdiagnostik_bohrkern::Entity",
        from = "Column::BohrkernId",
        to = "super::bauwerksdiagnostik_bohrkern::Column::Id"
    )]
    Bohrkern,
    #[sea_orm(
        belongs_to = "super::assetverwaltung_bauwerk::Entity",
        from = "Column::BauwerkIdId",
        to = "super::assetverwaltung_bauwerk::Column::Id"
    )]
    BauwerkId,
}

impl Related<super::assetverwaltung_projekt::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjektId.def()
    }
}

impl Related<super::bauwerksdiagnostik_bohrkern::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bohrkern.def()
    }
}

impl Related<super::assetverwaltung_bauwerk::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BauwerkId.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
