#[macro_use]
extern crate rocket;
use rocket::{http::Status, serde::json::Json};
use sea_orm_rocket::{Connection, Database};

mod pool;
use pool::Db;

mod entities;
use entities::*;

mod get;

#[get("/get_ftp_configuration/<id_type>/<id>")]
async fn get_ftp_configuration(
    conn: Connection<'_, Db>,
    id_type: String,
    id: i32,
) -> Result<Json<plotsystem_ftp_configurations::Model>, Status> {
    let db = conn.into_inner();

    return match id_type.as_str() {
        "ftp_id" => Ok(Json(get::ftp_configuration::by_ftp_id(db, id).await)),
        "server_id" => Ok(Json(get::ftp_configuration::by_server_id(db, id).await)),
        "cp_id" => Ok(Json(get::ftp_configuration::by_cp_id(db, id).await)),
        _ => Err(Status::BadRequest),
    };
}

#[get("/<bytes>")]
async fn byte_arr(bytes: String) -> Status {
    if bytes.as_bytes() == &[112_u8, 105_u8, 112_u8, 112_u8, 101_u8, 110_u8] {
        return Status::UnavailableForLegalReasons;
    } else {
        return Status::NotFound;
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![get_ftp_configuration, byte_arr])
}
