mod model;
mod schema;

use std::io;

use diesel::{query_dsl::methods::FilterDsl, Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use model::{Book, NewBook};
use schema::book_master::author;

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
        // self.add_a_book();
        self.display_all_books();
        // self.get_book_by_id();
        self.update_book();
    }

    fn add_a_book(&mut self) {
        use schema::book_master;

        println!("Enter the TITLE of the book: ");
        let mut title = String::new();
        io::stdin().read_line(&mut title).unwrap();
        let title = title.trim();

        println!("Enter the AUTHOR of the book: ");
        let mut author_name = String::new();
        io::stdin().read_line(&mut author_name).unwrap();
        let author_name = author_name.trim();

        println!("Enter the price of the book: ");
        let mut price = String::new();
        io::stdin().read_line(&mut price).unwrap();
        let price = price.trim().parse::<f64>().unwrap();

        let new_book = NewBook::new(title.to_string(), author_name.to_string(), price);

        diesel::insert_into(book_master::table)
            .values(&new_book)
            .get_result::<Book>(&mut self.database_connection)
            .expect("Error adding a book");
    }

    fn display_all_books(&mut self) {
        use schema::book_master::dsl::*;
        
        let books = book_master
            .load::<Book>(&mut self.database_connection)
            .expect("Error while displaying all books");
        println!("\n------------DISPLAY ALL BOOKS-------------\n");
        for book in books {
            println!("Title: {}", book.title);
            println!("Author: {}", book.author);
            println!("Price: {}", book.price);
            println!("-----------------------------");
        }
    }

    fn get_book_by_id(&mut self) {
        use schema::book_master::dsl::*;

        println!("Enter the id of the book, you wanna search: ");
        let mut book_id = String::new();
        io::stdin().read_line(&mut book_id).unwrap();
        let book_id = book_id.trim().parse::<i32>().unwrap();

        let book = book_master
            .find(book_id)
            .get_result::<Book>(&mut self.database_connection)
            .expect("Error getting a book by id");

        println!("\n----------BOOK DETAILS (ID #{})-----------\n", book_id);
        println!("Title: {}", book.title);
        println!("Author: {}", book.author);
        println!("Price: {}", book.price);
    }

    fn update_book(&mut self) {
        use schema::book_master::dsl::*;

        println!("Enter the id of the book, which you wanna update: ");
        let mut book_id = String::new();
        io::stdin().read_line(&mut book_id).unwrap();
        let book_id = book_id.trim().parse::<i32>().unwrap();
        
        println!("Update the title of the book: ");
        let mut updated_title = String::new();
        io::stdin().read_line(&mut updated_title).unwrap();
        
        println!("Update the author of the book: ");
        let mut updated_author = String::new();
        io::stdin().read_line(&mut updated_author).unwrap();

        println!("Update the price of the book: ");
        let mut updated_price = String::new();
        io::stdin().read_line(&mut updated_price).unwrap();
        let updated_price = updated_price.trim().parse::<f64>().unwrap();

        diesel::update(book_master.find(book_id))
            .set((
                title.eq(updated_title.trim()),
                author.eq(updated_author.trim()),
                price.eq(updated_price)
            ))
            .execute(&mut self.database_connection)
            .expect("Error in updating book");
    }
}