//! SeaORM entity for SQLite table `client_sync_changes` (migration 0007).
//! One row per changed field of a record that is not yet saved on the server.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "client_sync_changes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub table_name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub column_name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub row_pk: String,
    pub server_value: Json,
    pub new_value: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
