use sea_orm::{ConnectOptions, Database};

mod mylimplies;
mod prelude;

use prelude::*;

#[async_std::main]
async fn main() {
    let mut opt = ConnectOptions::new("mysql://root@localhost/sea_orm_test".to_owned());
    opt.sqlx_logging(true);

    let db = Database::connect(opt).await.unwrap();

    let a = Mylimplies::find_by_id(1).one(db).await?;
}
