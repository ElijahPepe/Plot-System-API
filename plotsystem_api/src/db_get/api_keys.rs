use std::process::exit;

use sea_orm::DatabaseConnection;

use crate::entities::{prelude::*, *};

use sea_orm::entity::*;

pub async fn by_api_key(db: &DatabaseConnection, api_key: &str) -> api_keys::Model {
    let api_key = ApiKeys::find_by_id(api_key.to_owned())
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    return api_key;
}

pub async fn api_key_exists(db: &DatabaseConnection, api_key: &str) -> bool {
    return match ApiKeys::find_by_id(api_key.to_owned()).one(db).await {
        Ok(m) => match m {
            Some(m) => true,
            None => false,
        },
        Err(e) => {
            print!("{:#?}", e);
            exit(0)
        }
    };
}
