//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, FromFormField,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "continent")]
pub enum Continent {
    #[sea_orm(string_value = "Europe")]
    Europe,
    #[sea_orm(string_value = "Asia")]
    Asia,
    #[sea_orm(string_value = "Africa")]
    Africa,
    #[sea_orm(string_value = "Oceania")]
    Oceania,
    #[sea_orm(string_value = "South America")]
    SouthAmerica,
    #[sea_orm(string_value = "North America")]
    NorthAmerica,
}
#[derive(
    Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, FromFormField,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "unclaimed")]
    Unclaimed,
    #[sea_orm(string_value = "unfinished")]
    Unfinished,
    #[sea_orm(string_value = "unreviewed")]
    Unreviewed,
    #[sea_orm(string_value = "finished")]
    Finished,
    #[sea_orm(string_value = "completed")]
    Completed,
}
