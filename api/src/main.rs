mod data_interface;

#[macro_use]
extern crate rocket;

#[get("/cityproject/<cp_id>")]
fn cityproject(cp_id: i64) -> String {
    "requested cityproject id ".to_string() + &cp_id.to_string()
}

#[get("/cityprojects")]
fn cityprojects() -> String {
    "requested a list of all cityprojects".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![cityproject, cityprojects])
}
