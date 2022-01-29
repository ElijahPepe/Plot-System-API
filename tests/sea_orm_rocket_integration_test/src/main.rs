#[macro_use]
extern crate rocket;
use rocket::{http::Status, serde::json::Json};

use std::process::exit;

use sea_orm::entity::*;
use sea_orm_rocket::{Connection, Database};

mod pool;
use pool::Db;

mod entities;
use entities::{prelude::*, *};

mod get;

#[get("/err/<id>")]
fn err(id: i32) -> Result<Json<i32>, Status> {
    if id == 1 {
        return Ok(Json(id));
    } else {
        return Err(Status::BadRequest);
    }
}

#[get("/get_ftp_configuration/<id_type>/<id>")]
async fn get_ftp_configuration(
    conn: Connection<'_, Db>,
    id_type: String,
    id: i32,
) -> Result<Json<plotsystem_ftp_configurations::Model>, Status> {
    let db = conn.into_inner();

    let a: plotsystem_ftp_configurations::Model = PlotsystemFtpConfigurations::find_by_id(id)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    println!("{:?}", a);

    return match id_type.as_str() {
        "ftp_id" => Ok(Json(get::ftp_configuration::by_ftp_id(db, id).await)),
        "server_id" => Ok(Json(get::ftp_configuration::by_server_id(db, id).await)), // Temporary
        "cp_id" => Ok(Json(get::ftp_configuration::by_cp_id(db, id).await)),         // Temporary
        _ => Err(Status::BadRequest),
    };
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![get_ftp_configuration, err])
}
