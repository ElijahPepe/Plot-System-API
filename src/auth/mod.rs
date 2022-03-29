pub mod auth_get_ftp_config_guard;
pub mod auth_preflag_request_guard;
pub mod auth_put_plot_request_guard;

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Unauthorized,
}

pub fn get_auth_key(req: &Request) -> Option<String> {
    return match req
        .headers()
        .get("authorization")
        .collect::<Vec<&str>>()
        .get(0)
    {
        Some(key) => Some(key.to_string()),
        None => None,
    };
}
