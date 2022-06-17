#[macro_use]
extern crate rocket;
use sea_orm_rocket::Database;

mod pool;
use pool::Db;

mod auth;
mod db_get;
mod entities;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::init()).mount(
        "/api/v1",
        routes![
            routes::get::ftp_configuration,
            routes::get::city_project,
            routes::get::city_projects,
            routes::get::server,
            routes::get::plot,
            routes::get::plots,
            routes::get::byte_arr,
            routes::post::plot_add,
            routes::put::set_pasted,
        ],
    )
}
