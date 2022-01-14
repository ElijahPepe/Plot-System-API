fn main() {}

// mod data_interface;

// #[macro_use]
// extern crate rocket;

// use rocket::serde::{json::Json, Serialize};

// #[get("/ftp_configuration/<id_type>/<id>")]
// fn ftp_configuration(id_type: String, id: i64) -> Json<data_interface::FtpConfiguration> {
//     match id_type.as_str() {
//         "cp_id" => {}
//         "ftp_id" => {}
//         "server_id" => {}
//     }

//     Json(data_interface::FtpConfiguration {})
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![ftp_configuration])
// }
