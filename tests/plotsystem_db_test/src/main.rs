use sea_orm::entity::*;
use sea_orm::Database;

mod entities;

use entities::{prelude::*, *};

#[async_std::main]
async fn main() {
    let db = Database::connect("mysql://root@localhost/plotsystem")
        .await
        .unwrap();

    let builders: Vec<plotsystem_builders::Model> =
        PlotsystemBuilders::find().all(&db).await.unwrap();

    // println!("{:?}", builders);

    println!(
        "{:?}",
        PlotsystemCityProjects::find_by_id(1)
            .one(&db)
            .await
            .unwrap()
    )
}
