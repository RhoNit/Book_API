use std::env;

use book_api::Database;
use dotenv::dotenv;



fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error while loading DATABASE_URL");
    let mut db = Database::new(database_url);
    db.run();
}
