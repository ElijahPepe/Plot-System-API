use sea_orm::entity::*;
use sea_orm::Database;

mod entities;

use entities::{prelude::*, *};

#[async_std::main]
async fn main() {
    let db = Database::connect("mysql://root@localhost/generate_test")
        .await
        .unwrap();

    let tests: Vec<test::Model> = Test::find().all(&db).await.unwrap();

    println!("{:?}", tests);
}
