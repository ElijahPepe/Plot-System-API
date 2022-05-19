use crate::entities::{prelude::*, *};
use sea_orm::DatabaseConnection;

use sea_orm::entity::*;

pub async fn by_server_id(db: &DatabaseConnection, server_id: i32) -> plotsystem_servers::Model {
    let server = PlotsystemServers::find_by_id(server_id)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    return server;
}

pub async fn by_country_id(db: &DatabaseConnection, country_id: i32) -> plotsystem_servers::Model {
    let server_id = super::country::by_country_id(db, country_id)
        .await
        .server_id;

    return by_server_id(db, server_id).await;
}
