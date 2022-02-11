use rocket::http::Status;
use rocket::outcome::Outcome;
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
        let db = req
            .guard::<Connection<'_, Db>>()
            .await
            .unwrap()
            .into_inner();

        let api_key = match req
            .headers()
            .get("authorization")
            .collect::<Vec<&str>>()
            .get(0)
        {
            Some(key) => key.to_string(),
            None => return Outcome::Failure((Status::BadRequest, AuthError::Missing)),
        };

        let plot_id = match req.uri().path().raw_segments().last() {
            Some(n) => n,
            None => return Outcome::Failure((Status::Unauthorized, AuthError::Unauthorized)),
        }
        .to_string()
        .to_owned()
        .parse::<i32>()
        .unwrap();

        print!("{:#?}", plot_id);

        let authorized_api_keys = crate::db_get::api_keys::by_plot_id(db, plot_id).await;

        return match authorized_api_keys
            .iter()
            .filter(|k| k.api_key == api_key)
            .collect::<Vec<&crate::entities::api_keys::Model>>()
            .len()
        {
            0 => Outcome::Failure((Status::Unauthorized, AuthError::Unauthorized)),
            _ => Outcome::Success(AuthPutGuard("Stuff".to_string())),
        };
    }
}
