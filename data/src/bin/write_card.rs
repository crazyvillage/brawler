use data::*;
use std::io::{stdin};

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("Enter card name");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    let card = create_post(connection, name);
    println!("\nSaved card {} with id {}", name, card.id);
}