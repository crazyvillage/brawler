// @generated automatically by Diesel CLI.

diesel::table! {
    card (id) {
        id -> Int4,
        #[max_length = 48]
        name -> Varchar,
    }
}

diesel::table! {
    card_color (id) {
        id -> Int4,
        card_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    color (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    commander (id) {
        id -> Int4,
        card_id -> Int4,
    }
}

diesel::table! {
    deck (id) {
        id -> Int4,
        commander_id -> Int4,
    }
}

diesel::table! {
    deck_card (id) {
        id -> Int4,
        deck_id -> Int4,
        card_id -> Int4,
    }
}

diesel::joinable!(card_color -> card (card_id));
diesel::joinable!(card_color -> color (color_id));
diesel::joinable!(commander -> card (card_id));
diesel::joinable!(deck -> commander (commander_id));
diesel::joinable!(deck_card -> card (card_id));
diesel::joinable!(deck_card -> deck (deck_id));

diesel::allow_tables_to_appear_in_same_query!(
    card,
    card_color,
    color,
    commander,
    deck,
    deck_card,
);
