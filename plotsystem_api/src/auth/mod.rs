// pub mod auth_post_plot_data_guard;
// pub mod auth_db_check;
pub mod auth_preflag_request_guard;
pub mod auth_put_plot_request_guard;

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Unauthorized,
}
