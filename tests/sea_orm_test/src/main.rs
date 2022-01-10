use sea_orm::{ConnectOptions, Database};

#[async_std::main]
async fn main() {
    let mut opt = ConnectOptions::new("mysql://root@localhost/sea_orm_test".to_owned());
    opt.sqlx_logging(true);

    let db = Database::connect(opt).await.unwrap();
}
