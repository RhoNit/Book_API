use diesel::deserialize::Queryable;
use diesel::Insertable;

use crate::schema::book_master;

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub price: f64,
}

#[derive(Insertable)]
#[diesel(table_name = book_master)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub price: f64,
}

impl NewBook {
    pub fn new(title: String, author: String, price: f64) -> NewBook {
        NewBook { 
            title, 
            author, 
            price
        }
    }
}