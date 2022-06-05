use sea_orm::{Condition, DatabaseConnection, JoinType, QueryFilter, QuerySelect};

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_ftp_id(
    db: &DatabaseConnection,
    ftp_id: i32,
) -> plotsystem_ftp_configurations::Model {
    let ftp = PlotsystemFtpConfigurations::find_by_id(ftp_id)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    return ftp;
}

pub async fn by_server_id(
    db: &DatabaseConnection,
    server_id: i32,
) -> plotsystem_ftp_configurations::Model {
    plotsystem_ftp_configurations::Entity::find()
        .join(
            JoinType::InnerJoin,
            plotsystem_ftp_configurations::Relation::PlotsystemServers.def(),
        )
        .filter(Condition::all().add(plotsystem_servers::Column::Id.eq(server_id)))
        .one(db)
        .await
        .unwrap()
        .unwrap()
}

pub async fn by_cp_id(db: &DatabaseConnection, cp_id: i32) -> plotsystem_ftp_configurations::Model {
    plotsystem_ftp_configurations::Entity::find()
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
        .await
        .unwrap()
        .unwrap()
}
