use self::models::*;
use diesel::prelude::*;
use data::*;

fn main() {
    use self::schema::card::dsl::*;

    let connection = &mut establish_connection();
    let results = card
        .limit(5)
        .select(Card::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} cards", results.len());
    for c in results {
        println!("{}", c.name);
    }
}