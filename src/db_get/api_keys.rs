use std::process::exit;

use sea_orm::{Condition, DatabaseConnection, JoinType, QueryFilter, QuerySelect};

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn api_key_exists(db: &DatabaseConnection, api_key: &str) -> bool {
    return match PlotsystemApiKeys::find()
        .filter(Condition::all().add(plotsystem_api_keys::Column::ApiKey.eq(api_key)))
        .one(db)
        .await
    {
        Ok(m) => match m {
            Some(_) => true,
            None => false,
        },
        Err(e) => {
            print!("{:#?}", e);
            exit(0)
        }
    };
}

pub async fn cp_related_to_api_key(db: &DatabaseConnection, api_key: String, cp_id: i32) -> bool {
    let matches = plotsystem_api_keys::Entity::find()
        .join(
            JoinType::InnerJoin,
            plotsystem_api_keys::Relation::PlotsystemBuildteams.def(),
        )
        .join(
            JoinType::InnerJoin,
            plotsystem_buildteams::Relation::PlotsystemBuildteamHasCountries.def(),
        )
        .join(
            JoinType::InnerJoin,
            plotsystem_buildteam_has_countries::Relation::PlotsystemCountries.def(),
        )
        .join(
            JoinType::InnerJoin,
            plotsystem_countries::Relation::PlotsystemCityProjects.def(),
        )
        .filter(
            Condition::all()
                .add(plotsystem_api_keys::Column::ApiKey.eq(api_key))
                .add(plotsystem_city_projects::Column::Id.eq(cp_id)),
        )
        .all(db)
        .await
        .unwrap();

    match matches.len() {
        1 => true,
        _ => false,
    }
}

pub async fn plot_related_to_api_key(
    db: &DatabaseConnection,
    api_key: String,
    plot_id: i32,
) -> bool {
    let matches = plotsystem_api_keys::Entity::find()
        .join(
            JoinType::InnerJoin,
            plotsystem_api_keys::Relation::PlotsystemBuildteams.def(),
        )
        .join(
            JoinType::InnerJoin,
            plotsystem_buildteams::Relation::PlotsystemBuildteamHasCountries.def(),
        )
        .join(
            JoinType::InnerJoin,
            plotsystem_buildteam_has_countries::Relation::PlotsystemCountries.def(),
        )
        .join(
            JoinType::InnerJoin,
            plotsystem_countries::Relation::PlotsystemCityProjects.def(),
        )
        .join(
            JoinType::InnerJoin,
            plotsystem_city_projects::Relation::PlotsystemPlots.def(),
        )
        .filter(
            Condition::all()
                .add(plotsystem_api_keys::Column::ApiKey.eq(api_key.to_owned()))
                .add(plotsystem_plots::Column::Id.eq(plot_id)),
        )
        .all(db)
        .await
        .unwrap();

    match matches.len() {
        1 => true,
        _ => false,
    }
}

pub async fn by_server_id(db: &DatabaseConnection, server_id: i32) -> Vec<api_keys::Model> {
    let country_ids = super::country::by_server_id(db, server_id).await;

    return None;
}
