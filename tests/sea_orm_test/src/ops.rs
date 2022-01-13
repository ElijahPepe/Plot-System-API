use super::*;
use sea_orm::{entity::*, error::*, query::*, DbConn, FromQueryResult};

pub async fn all_about_select(db: &DbConn) -> Result<(), DbErr> {
    find_all(db).await?;

    println!("===== =====\n");

    find_together(db).await?;

    println!("===== =====\n");

    find_one(db).await?;

    println!("===== =====\n");

    count_fruits_by_cake(db).await?;

    println!("===== =====\n");

    find_many_to_many(db).await?;

    if false {
        println!("===== =====\n");

        all_about_select_json(db).await?;
    }

    println!("===== =====\n");

    find_all_stream(&db).await.unwrap();

    println!("===== =====\n");

    find_first_page(&db).await.unwrap();

    println!("===== =====\n");

    find_num_pages(&db).await.unwrap();

    Ok(())
}