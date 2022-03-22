//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "plotsystem_ftp_configurations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub schematics_path: Option<String>,
    pub address: String,
    pub port: i32,
    #[sea_orm(column_name = "isSFTP")]
    pub is_sftp: i8,
    pub username: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::plotsystem_servers::Entity")]
    PlotsystemServers,
}

impl Related<super::plotsystem_servers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlotsystemServers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
