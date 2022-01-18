use sea_orm::entity::*;
use sea_orm::Database;

mod prelude;
mod sea_orm_active_enums;
mod test;

use prelude::*;

#[async_std::main]
async fn main() {
    let db = Database::connect("mysql://root@localhost/plotsystem")
        .await
        .unwrap();

    let tests: Vec<test::Model> = Test::find().all(&db).await.unwrap();

    println!("{:?}", tests);
}
