use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use sea_orm_rocket::Connection;

use super::AuthError;

use crate::pool::Db;

// this will check if the api key is even present in the database

#[derive(Debug)]
pub struct AuthPreflag(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthPreflag {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let api_key = match req
            .headers()
            .get("authorization")
            .collect::<Vec<&str>>()
            .get(0)
        {
            Some(key) => key.to_string(),
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        };

        let db = req
            .guard::<Connection<'_, Db>>()
            .await
            .unwrap()
            .into_inner();

        return match crate::db_get::api_keys::api_key_exists(db, &api_key).await {
            true => Outcome::Success(AuthPreflag(api_key)),
            false => Outcome::Failure((Status::Unauthorized, AuthError::Unauthorized)),
        };
    }
}
