use rocket::data::{self, ByteUnit, Data, FromData, ToByteUnit};
use rocket::request::Request;
use rocket_dyn_templates::tera::Test;

struct TestData {
    a: i32,
    b: bool,
}

#[derive(Debug)]
enum Err {
    Yes,
    No,
}

// Implementing this would be the correct method, but it seems unneccesarily complex. For now I will go with the easy route.
// Implements this in the future.

#[rocket::async_trait]
impl<'r> FromData<'r> for TestData {
    type Error = Err;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        let a = data.open(20.kilobytes());

        print!("{:#?}", a);
    }
}
