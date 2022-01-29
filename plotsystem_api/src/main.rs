#[macro_use]
extern crate rocket;
use sea_orm_rocket::Database;

mod pool;
use pool::Db;

mod entities;

mod db_get;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::init()).mount(
        "/api/v1",
        routes![
            routes::get::get_ftp_configuration,
            routes::get::get_city_project,
            routes::get::get_city_projects,
            routes::get::get_server,
            routes::get::byte_arr
        ],
    )
}
