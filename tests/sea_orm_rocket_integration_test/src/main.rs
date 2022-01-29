#[macro_use]
extern crate rocket;

use std::process::exit;

use rocket::serde::json::Json;

use sea_orm::entity::*;
use sea_orm_rocket::{Connection, Database};

mod pool;
use pool::Db;

mod entities;

use entities::{prelude::*, *};

#[get("/get_ftp_configuration/<id_type>/<id>")]
async fn get_ftp_configuration(
    conn: Connection<'_, Db>,
    id_type: String,
    id: i32,
) -> Json<plotsystem_ftp_configurations::Model> {
    let db = conn.into_inner();

    return Json(
        match id_type.as_str() {
            "ftp_id" => PlotsystemFtpConfigurations::find_by_id(id).one(db),
            "cp_id" => PlotsystemFtpConfigurations::find_by_id(id).one(db), // Temporary
            "server_id" => exit(id),                                        // Temporary
            _ => exit(id),
        }
        .await
        .unwrap()
        .unwrap()
        .into(),
    );
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![get_ftp_configuration])
}
