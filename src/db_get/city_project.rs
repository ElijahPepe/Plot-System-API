use sea_orm::DatabaseConnection;

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_cp_id(db: &DatabaseConnection, cp_id: i32) -> plotsystem_city_projects::Model {
    PlotsystemCityProjects::find_by_id(cp_id)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into()
}

pub async fn all(db: &DatabaseConnection) -> Vec<plotsystem_city_projects::Model> {
    PlotsystemCityProjects::find().all(db).await.unwrap()
}
