use sea_orm::entity::*;
use sea_orm::Database;

mod cake;
mod cake_filling;
mod filling;
mod fruit;
mod prelude;
// mod operation;
// mod select;

use prelude::*;

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
