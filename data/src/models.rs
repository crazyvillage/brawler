use diesel::prelude::*;
use crate::schema::card;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::card)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Card {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = card)]
pub struct NewCard<'a> {
    pub name: &'a str,
}