use crate::auth::{auth_get_ftp_config_guard::FtpAuth, auth_preflag_request_guard::AuthPreflag};
use crate::{db_get, entities::*, pool::Db};
use rocket::{http::Status, serde::json::Json};
use sea_orm_rocket::Connection;

#[get("/get_ftp_configuration/<id_type>/<id>")]
pub async fn get_ftp_configuration(
    conn: Connection<'_, Db>,
    _auth_preflag: AuthPreflag,
    _auth: FtpAuth,
    id_type: String,
    id: i32,
) -> Result<Json<plotsystem_ftp_configurations::Model>, Status> {
    let db = conn.into_inner();

    match match id_type.as_str() {
        "ftp_id" => db_get::ftp_configuration::by_ftp_id(db, id).await,
        "server_id" => db_get::ftp_configuration::by_server_id(db, id).await,
        "cp_id" => db_get::ftp_configuration::by_cp_id(db, id).await,
        _ => return Err(Status::BadRequest),
    } {
        Ok(ftp_configuration) => Ok(Json(ftp_configuration)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/get_city_project/<id>")]
pub async fn get_city_project(
    conn: Connection<'_, Db>,
    _auth_preflag: AuthPreflag,
    id: i32,
) -> Result<Json<plotsystem_city_projects::Model>, Status> {
    let db = conn.into_inner();

    match db_get::city_project::by_cp_id(db, id).await {
        Ok(cp) => Ok(Json(cp)),
        // Return error message in status
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/get_city_projects")]
pub async fn get_city_projects(
    conn: Connection<'_, Db>,
    _auth_preflag: AuthPreflag,
) -> Result<Json<Vec<plotsystem_city_projects::Model>>, Status> {
    let db = conn.into_inner();

    match db_get::city_project::all(db).await {
        Ok(cp) => Ok(Json(cp)),
        // Return error message in status
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/get_server/<id_type>/<id>")]
pub async fn get_server(
    conn: Connection<'_, Db>,
    _auth_preflag: AuthPreflag,
    id_type: String,
    id: i32,
) -> Result<Json<plotsystem_servers::Model>, Status> {
    let db = conn.into_inner();

    match match id_type.as_str() {
        "server_id" => db_get::server::by_server_id(db, id).await,
        "country_id" => db_get::server::by_country_id(db, id).await,
        _ => return Err(Status::BadRequest),
    } {
        Ok(server) => Ok(Json(server)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/get_plot/<plot_id>")]
pub async fn get_plot(
    conn: Connection<'_, Db>,
    _auth_preflag: AuthPreflag,
    plot_id: i32,
) -> Result<Json<plotsystem_plots::Model>, Status> {
    let db = conn.into_inner();

    match db_get::plot::by_plot_id(db, plot_id).await {
        Ok(plot) => Ok(Json(plot)),
        // Return error message in status
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/get_plots?<status>&<pasted>&<limit>")]
pub async fn get_plots(
    conn: Connection<'_, Db>,
    _auth_preflag: AuthPreflag,
    status: Option<sea_orm_active_enums::Status>,
    pasted: Option<bool>,
    limit: Option<u32>,
) -> Result<Json<Vec<plotsystem_plots::Model>>, Status> {
    let db = conn.into_inner();

    match db_get::plot::filtered(db, status, pasted, limit).await {
        Ok(plots) => Ok(Json(plots)),
        // Return error message in status
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/<bytes>")]
pub async fn byte_arr(bytes: String) -> Status {
    return match bytes.as_bytes() == &[112_u8, 105_u8, 112_u8, 112_u8, 101_u8, 110_u8] {
        true => Status::ExpectationFailed,
        false => Status::NotFound,
    };
}
