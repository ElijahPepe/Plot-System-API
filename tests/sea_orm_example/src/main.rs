use sea_orm::entity::*;
use sea_orm::Database;

mod entities;
mod example_cake;
mod example_cake_filling;
mod example_filling;
mod example_fruit;
// mod operation;
// mod select;

use entities::*;
use example_cake as cake;
use example_cake_filling as cake_filling;
use example_filling as filling;
use example_fruit as fruit;

#[async_std::main]
async fn main() {
    let db = Database::connect("mysql://root@localhost/sea_orm_example")
        .await
        .unwrap();

    let cakes: Vec<cake::Model> = Cake::find().all(&db).await.unwrap();

    println!("{:?}\n", cakes);

    // println!("{:?}\n", db);

    // println!("===== =====\n");

    // all_about_select(&db).await.unwrap();

    // println!("===== =====\n");

    // all_about_operation(&db).await.unwrap();
}
