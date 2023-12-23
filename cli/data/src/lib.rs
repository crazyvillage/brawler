pub mod models;
pub mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewCard};
use external::models::ScryfallCard;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_cards(conn: &mut PgConnection, cards: &Vec<ScryfallCard>) -> () {
    use crate::schema::card;

    let new_cards: Vec<NewCard> = cards.into_iter().map(|c|  NewCard{name: &c.name}).collect();

    diesel::insert_into(card::table)
        .values(&new_cards)
        .execute(conn)
        .expect("Error inserting cards");
}