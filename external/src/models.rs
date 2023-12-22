use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScryfallResponse {
    pub has_more: bool,
    pub next_page: String,
    pub data: Vec<ScryfallCard>
}

#[derive(Deserialize)]
pub struct ScryfallCard {
    pub name: String,
    pub type_line: String,
    pub color_identity: Vec<String>
}