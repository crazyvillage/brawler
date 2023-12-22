use external::get_cards;

fn main() {
    let resp = get_cards().unwrap();
    for card in resp.data {
        println!("{}", card.name)
    }
}