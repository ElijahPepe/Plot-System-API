use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::outcome::Outcome::Success;
use rocket::request::{self, FromRequest, Request};
use sea_orm_rocket::Connection;

use super::AuthError;

use crate::pool::Db;

#[derive(Debug)]
pub struct AuthPutGuard(String);

// this feels insecure, this may be vulnerable to url xss attacks, not sure tho.
// Requires Further Testing

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthPutGuard {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db = match req.guard::<Connection<'_, Db>>().await {
            Success(db) => db.into_inner(),
            _ => return Outcome::Failure((Status::BadRequest, AuthError::Invalid)),
        };

        let api_key = match req
            .headers()
            .get("authorization")
            .collect::<Vec<&str>>()
            .get(0)
        {
            Some(key) => key.to_string(),
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        };

        let plot_id = match match req.uri().path().raw_segments().last() {
            Some(n) => n,
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        }
        .to_string()
        .parse::<i32>()
        {
            Ok(id) => id,
            Err(_) => return Outcome::Failure((Status::BadRequest, AuthError::Invalid)),
        };

        match match crate::db_get::api_keys::plot_related_to_api_key(db, &api_key, plot_id).await {
            Ok(exists) => exists,
            Err(e) => {
                return Outcome::Failure((
                    Status::BadRequest,
                    AuthError::DataBaseError(e.to_string()),
                ))
            }
        } {
            true => Outcome::Success(AuthPutGuard(api_key)),
            false => Outcome::Failure((Status::Unauthorized, AuthError::Unauthorized)),
        }
    }
}
