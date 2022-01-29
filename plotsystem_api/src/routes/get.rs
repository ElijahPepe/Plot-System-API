use crate::{db_get, entities::*, pool::Db};
use rocket::{http::Status, serde::json::Json};
use sea_orm_rocket::Connection;

#[get("/get_ftp_configuration/<id_type>/<id>")]
pub async fn get_ftp_configuration(
    conn: Connection<'_, Db>,
    id_type: String,
    id: i32,
) -> Result<Json<plotsystem_ftp_configurations::Model>, Status> {
    let db = conn.into_inner();

    return match id_type.as_str() {
        "ftp_id" => Ok(Json(db_get::ftp_configuration::by_ftp_id(db, id).await)),
        "server_id" => Ok(Json(db_get::ftp_configuration::by_server_id(db, id).await)),
        "cp_id" => Ok(Json(db_get::ftp_configuration::by_cp_id(db, id).await)),
        _ => Err(Status::BadRequest),
    };
}

#[get("/get_city_project/<id>")]
pub async fn get_city_project(
    conn: Connection<'_, Db>,
    id: i32,
) -> Result<Json<plotsystem_city_projects::Model>, Status> {
    let db = conn.into_inner();

    return Ok(Json(db_get::city_project::by_cp_id(db, id).await));
}

#[get("/get_city_projects")]
pub async fn get_city_projects(
    conn: Connection<'_, Db>,
) -> Result<Json<Vec<plotsystem_city_projects::Model>>, Status> {
    let db = conn.into_inner();

    return Ok(Json(db_get::city_project::all(db).await));
}

#[get("/get_server/<id_type>/<id>")]
pub async fn get_server(
    conn: Connection<'_, Db>,
    id_type: String,
    id: i32,
) -> Result<Json<plotsystem_servers::Model>, Status> {
    let db = conn.into_inner();

    return match id_type.as_str() {
        "server_id" => Ok(Json(db_get::server::by_server_id(db, id).await)),
        "country_id" => Ok(Json(db_get::server::by_country_id(db, id).await)),
        _ => Err(Status::BadRequest),
    };
}

#[get("/<bytes>")]
pub async fn byte_arr(bytes: String) -> Status {
    if bytes.as_bytes() == &[112_u8, 105_u8, 112_u8, 112_u8, 101_u8, 110_u8] {
        return Status::UnavailableForLegalReasons;
    } else {
        return Status::NotFound;
    }
}
