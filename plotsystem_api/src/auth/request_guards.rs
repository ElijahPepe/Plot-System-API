use rocket::request::{self, FromRequest, Request};

// this will check if the api key is even present in the database

struct AuthPreflag {}

// #[rocket::async_trait]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthPreflag {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        /* .. */
    }
}
