use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use sea_orm_rocket::Connection;
use rocket::outcome::Outcome::Success;

use crate::pool::Db;

use super::{get_auth_key, AuthError};

use crate::db_get::api_keys::{
    cp_related_to_api_key, ftp_configuration_related_to_api_key, server_related_to_api_key,
};

#[derive(Debug)]
pub struct FtpAuth(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for FtpAuth {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let api_key = match get_auth_key(req) {
            Some(key) => key,
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        };

        println!("{}", api_key);

        let db = match req
            .guard::<Connection<'_, Db>>()
            .await {
                Success(db) => db.into_inner(),
                _ => return Outcome::Failure((Status::BadRequest, AuthError::Invalid)),
            };

        let id_type = match req
            .uri()
            .path()
            .raw_segments()
            .nth(req.uri().path().raw_segments().count() - 2)
        {
            Some(id_type) => id_type.to_string().to_owned(),
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        };

        println!("{}", id_type);

        let id = match match req.uri().path().raw_segments().last() {
            Some(n) => n,
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        }
        .to_string()
        .parse::<i32>()
        {
            Ok(id) => id,
            Err(_) => return Outcome::Failure((Status::BadRequest, AuthError::Invalid)),
        };

        println!("{}", id);

        match match match id_type.as_str() {
            "ftp_id" => ftp_configuration_related_to_api_key(db, &api_key, id).await,
            "server_id" => server_related_to_api_key(db, &api_key, id).await,
            "cp_id" => cp_related_to_api_key(db, &api_key, id).await,
            _ => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
            
        }
        // This should work but doesn't 🤷‍♂️
        //.await 
        {
            Ok(exists) => exists,
            Err(e) => return Outcome::Failure((Status::BadRequest, AuthError::DataBaseError(e.to_string()))),
        } {
            true => Outcome::Success(FtpAuth(api_key.to_owned())),
            false => Outcome::Failure((Status::Unauthorized, AuthError::Unauthorized)),
        }
    }
}
