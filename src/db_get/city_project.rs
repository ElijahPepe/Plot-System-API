use sea_orm::{DatabaseConnection, DbErr};

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_cp_id(
    db: &DatabaseConnection,
    cp_id: i32,
) -> Result<plotsystem_city_projects::Model, DbErr> {
    match PlotsystemCityProjects::find_by_id(cp_id).one(db).await? {
        Some(cp) => Ok(cp),
        None => Err(DbErr::RecordNotFound(format!(
            "City project with id {} does not exists",
            cp_id
        ))),
    }
}

pub async fn all(db: &DatabaseConnection) -> Result<Vec<plotsystem_city_projects::Model>, DbErr> {
    PlotsystemCityProjects::find().all(db).await
}
