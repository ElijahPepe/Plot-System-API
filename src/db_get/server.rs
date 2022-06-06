use sea_orm::{Condition, DatabaseConnection, DbErr, JoinType, QueryFilter, QuerySelect};

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_server_id(
    db: &DatabaseConnection,
    server_id: i32,
) -> Result<plotsystem_servers::Model, DbErr> {
    match PlotsystemServers::find_by_id(server_id).one(db).await? {
        Some(server) => Ok(server),
        None => Err(DbErr::RecordNotFound(format!(
            "Server with id {} does not exists",
            server_id
        ))),
    }
}

pub async fn by_country_id(
    db: &DatabaseConnection,
    country_id: i32,
) -> Result<plotsystem_servers::Model, DbErr> {
    match plotsystem_servers::Entity::find()
        .join(
            JoinType::InnerJoin,
            plotsystem_servers::Relation::PlotsystemCountries.def(),
        )
        .filter(Condition::all().add(plotsystem_countries::Column::Id.eq(country_id)))
        .one(db)
        .await?
    {
        Some(server) => Ok(server),
        None => Err(DbErr::RecordNotFound(format!(
            "Server with country id {} does not exists",
            country_id
        ))),
    }
}
