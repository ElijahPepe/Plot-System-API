pub mod auth_access_request_guard;
pub mod auth_preflag_request_guard;

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Unauthorized,
}
