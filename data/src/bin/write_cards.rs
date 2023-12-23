use data::*;
use external::get_cards;

fn main() {
    let connection = &mut establish_connection();

    let resp = get_cards().unwrap();
    create_cards(connection, &resp.data);
    println!("Finished saving cards!");
}