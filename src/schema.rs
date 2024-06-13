// @generated automatically by Diesel CLI.

diesel::table! {
    book_master (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 50]
        author -> Varchar,
        price -> Float8,
    }
}
