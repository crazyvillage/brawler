pub mod models;
pub mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewCard, Card};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_card(conn: &mut PgConnection, name: &str) -> Card {
    use crate::schema::card;

    let new_card = NewCard { name };

    diesel::insert_into(card::table)
        .values(&new_card)
        .returning(Card::as_returning())
        .get_result(conn)
        .expect("Error saving new card")
}