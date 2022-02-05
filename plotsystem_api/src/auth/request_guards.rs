use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

// this will check if the api key is even present in the database

#[derive(Debug)]
pub struct AuthPreflag(String);

#[derive(Debug)]
pub enum PreflagError {
    Missing,
    Invalid,
}

fn is_valid(key: &str) -> bool {
    key == "valid_key"
}

// #[rocket::async_trait]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthPreflag {
    type Error = PreflagError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let key: String = req.headers().get("authorisation").collect::<Vec<&str>>()[0].to_string();

        return match is_valid(&key) {
            true => Outcome::Success(AuthPreflag(key)),
            false => Outcome::Failure((Status::Unauthorized, PreflagError::Invalid)),
        };
    }
}
