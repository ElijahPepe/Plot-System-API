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

    println!("ftp id is {:?}", server);

    return server;
}
