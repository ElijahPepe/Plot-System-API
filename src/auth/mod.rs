pub mod auth_get_ftp_config_guard;
pub mod auth_preflag_request_guard;
pub mod auth_put_plot_request_guard;

use rocket::request::Request;

#[derive(Debug)]
pub enum AuthError {
    DataBaseError(String),
    Missing,
    Unauthorized,
    Invalid,
}

pub fn get_auth_key(req: &Request) -> Option<String> {
    req.headers()
        .get("authorization")
        .collect::<Vec<&str>>()
        .get(0)
        .map(|key| key.to_string())
}
