use sea_orm::{Condition, DatabaseConnection, DbErr, JoinType, QueryFilter, QuerySelect};

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_ftp_id(
    db: &DatabaseConnection,
    ftp_id: i32,
) -> Result<plotsystem_ftp_configurations::Model, DbErr> {
    match PlotsystemFtpConfigurations::find_by_id(ftp_id)
        .one(db)
        .await?
    {
        Some(ftp) => Ok(ftp),
        None => Err(DbErr::RecordNotFound(format!(
            "FTP configuration with id {} does not exists",
            ftp_id
        ))),
    }
}

pub async fn by_server_id(
    db: &DatabaseConnection,
    server_id: i32,
) -> Result<plotsystem_ftp_configurations::Model, DbErr> {
    match plotsystem_ftp_configurations::Entity::find()
        .join(
            JoinType::InnerJoin,
            plotsystem_ftp_configurations::Relation::PlotsystemServers.def(),
        )
        .filter(Condition::all().add(plotsystem_servers::Column::Id.eq(server_id)))
        .one(db)
        .await?
    {
        Some(ftp) => Ok(ftp),
        None => Err(DbErr::RecordNotFound(format!(
            "FTP configuration with server id {} does not exists",
            server_id
        ))),
    }
}

pub async fn by_cp_id(
    db: &DatabaseConnection,
    cp_id: i32,
) -> Result<plotsystem_ftp_configurations::Model, DbErr> {
    match plotsystem_ftp_configurations::Entity::find()
        .join(
            JoinType::InnerJoin,
            plotsystem_ftp_configurations::Relation::PlotsystemServers.def(),
        )
        .join(
            JoinType::InnerJoin,
            plotsystem_servers::Relation::PlotsystemCountries.def(),
        )
        .join(
            JoinType::InnerJoin,
            plotsystem_countries::Relation::PlotsystemCityProjects.def(),
        )
        .filter(Condition::all().add(plotsystem_city_projects::Column::Id.eq(cp_id)))
        .one(db)
        .await?
    {
        Some(ftp) => Ok(ftp),
        None => Err(DbErr::RecordNotFound(format!(
            "FTP configuration with city project id {} does not exists",
            cp_id
        ))),
    }
}
