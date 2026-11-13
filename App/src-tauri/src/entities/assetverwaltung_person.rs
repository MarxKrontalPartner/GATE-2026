//! SeaORM entity for SQLite table `assetverwaltung_person`.
//! Schema-derived from the GATE local SQL migrations.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "assetverwaltung_person")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub vorname: String,
    pub stellenbezeichnung: String,
    pub email: String,
    pub telefon: String,
    pub firma_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assetverwaltung_firma::Entity",
        from = "Column::FirmaId",
        to = "super::assetverwaltung_firma::Column::Id"
    )]
    Firma,
}

impl Related<super::assetverwaltung_firma::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Firma.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
