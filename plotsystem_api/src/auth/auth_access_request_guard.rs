use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use sea_orm_rocket::Connection;

use super::AuthError;

use crate::pool::Db;

#[derive(Debug)]
pub struct AuthAccess(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthAccess {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let uri = req.uri();

        match uri.path {
          ""
        }
    }
}
