mod models;

use crate::models::ScryfallResponse;


pub fn get_cards() -> Result<ScryfallResponse, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://api.scryfall.com/cards/search?q=f:historic")?
        .json::<ScryfallResponse>()?;
    Ok(resp)
}