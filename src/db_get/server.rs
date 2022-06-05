use sea_orm::{Condition, DatabaseConnection, JoinType, QueryFilter, QuerySelect};

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_server_id(db: &DatabaseConnection, server_id: i32) -> plotsystem_servers::Model {
    PlotsystemServers::find_by_id(server_id)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into()
}

pub async fn by_country_id(db: &DatabaseConnection, country_id: i32) -> plotsystem_servers::Model {
    plotsystem_servers::Entity::find()
        .join(
            JoinType::InnerJoin,
            plotsystem_servers::Relation::PlotsystemCountries.def(),
        )
        .filter(Condition::all().add(plotsystem_countries::Column::Id.eq(country_id)))
        .one(db)
        .await
        .unwrap()
        .unwrap()
}
