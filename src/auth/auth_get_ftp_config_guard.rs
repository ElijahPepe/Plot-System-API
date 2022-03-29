use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use sea_orm_rocket::Connection;

use crate::pool::Db;

use super::{get_auth_key, AuthError};

#[derive(Debug)]
pub struct AuthPreflag(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthPreflag {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let key = match get_auth_key(req) {
            Some(key) => key,
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        };

        let db = req
            .guard::<Connection<'_, Db>>()
            .await
            .unwrap()
            .into_inner();

        let id_type = match req
            .uri()
            .path()
            .raw_segments()
            .nth(req.uri().path().raw_segments().count())
        {
            Some(id_type) => id_type.to_string().to_owned(),
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        };

        let id = match req.uri().path().raw_segments().last() {
            Some(n) => n,
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        }
        .to_string()
        .to_owned()
        .parse::<i32>()
        .unwrap();

        let authorized_api_keys = match id_type {
          "ftp_id" => ,
          "server_id" => ,
          "cp_id" => ,
          _ => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        };
    }
}
