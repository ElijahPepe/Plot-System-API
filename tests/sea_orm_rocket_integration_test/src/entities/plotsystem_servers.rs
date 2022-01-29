//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "plotsystem_servers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub ftp_configuration_id: Option<i32>,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::plotsystem_ftp_configurations::Entity",
        from = "Column::FtpConfigurationId",
        to = "super::plotsystem_ftp_configurations::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    PlotsystemFtpConfigurations,
    #[sea_orm(has_many = "super::plotsystem_countries::Entity")]
    PlotsystemCountries,
}

impl Related<super::plotsystem_ftp_configurations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlotsystemFtpConfigurations.def()
    }
}

impl Related<super::plotsystem_countries::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlotsystemCountries.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
