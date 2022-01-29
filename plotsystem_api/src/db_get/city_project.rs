use sea_orm::DatabaseConnection;

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_cp_id(db: &DatabaseConnection, cp_id: i32) -> plotsystem_city_projects::Model {
    let city_project = PlotsystemCityProjects::find_by_id(cp_id)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    return city_project;
}

pub async fn all(db: &DatabaseConnection) -> Vec<plotsystem_city_projects::Model> {
    let city_project = PlotsystemCityProjects::find().all(db).await.unwrap();

    return city_project;
}
