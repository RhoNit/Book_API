mod model;
mod schema;

use std::io;

use diesel::{Connection, PgConnection, RunQueryDsl};
use model::{Book, NewBook};

pub struct Database {
    pub database_connection: PgConnection,
}

impl Database {
    pub fn new(database_url: String) -> Database {
        let db_conn = PgConnection::establish(&database_url).expect("Failed creating DB_CONN");
        Database {
            database_connection: db_conn,
        }
    }

    pub fn run(&mut self) {
        self.add_a_book();
    }

    fn add_a_book(&mut self) {
        use schema::book_master;

        println!("Enter the TITLE of the book: ");
        let mut title = String::new();
        io::stdin().read_line(&mut title).unwrap();
        let title = title.trim();

        println!("Enter the AUTHOR of the book: ");
        let mut author = String::new();
        io::stdin().read_line(&mut author).unwrap();
        let author = author.trim();

        println!("Enter the price of the book: ");
        let mut price = String::new();
        io::stdin().read_line(&mut price).unwrap();
        let price = price.trim().parse::<f64>().unwrap();

        let new_book = NewBook::new(title.to_string(), author.to_string(), price);

        diesel::insert_into(book_master::table)
            .values(&new_book)
            .get_result::<Book>(&mut self.database_connection)
            .expect("Error adding a book");
    }
}