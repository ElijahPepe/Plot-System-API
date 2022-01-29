use sea_orm::DatabaseConnection;

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_country_id(
    db: &DatabaseConnection,
    country_id: i32,
) -> plotsystem_countries::Model {
    let country = PlotsystemCountries::find_by_id(country_id)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    return country;
}

pub async fn by_cp_id(db: &DatabaseConnection, cp_id: i32) -> plotsystem_countries::Model {
    let country_id = super::city_project::by_cp_id(db, cp_id).await.country_id;

    let country = by_country_id(db, country_id).await;

    return country;
}
