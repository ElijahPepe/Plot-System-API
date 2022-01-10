use self::models::*;
use diesel::prelude::*;
use diesel_tutorial_2::*;

fn main() {
    use self::schema::plotsystem_difficulties::dsl::*;

    let connection = &mut establish_connection();
    let results = plotsystem_difficulties
        .limit(5)
        .load::<Difficulty>(connection)
        .expect("Error loading posts");

    println!("Displaying {} difficulties", results.len());
    for post in results {
        println!("{:?}", post)
    }
}
