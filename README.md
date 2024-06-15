## Getting started with Diesel🦀
- Single-entity based CRUD
- Pure `Rust` (no framework is used)
- DB: `PostgreSQL`
- ORM: `Diesel`

## Project Setup
### 1. Clone the repository
`git clone --depth 1 -b master https://github.com/RhoNit/Book_API`<br>
`cd Book_API`
<hr>

### 2. Setup Diesel CLI & installation of crates/dependencies
- If you don't have `diesel_cli` installed in your machine, please install it first:<br>
  `cargo install diesel_cli --no-default-features --features postgres`
- Install all the crates defined inside `Cargo.toml`<br>
  `cargo build`
<hr>

### 3. Define Database Config inside .env & Diesel Setup
- Define the `DATABASE_URL` inside `.env` file. For instance, <br>`DATABASE_URL=postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{HOST}:{PORT}/{DB_NAME}`
- Then setup diesel: `diesel setup`
<hr>

### 4. Generate migration and update up.sql and down.sql file content
- Create `book` table by adding a migration:<br> `diesel migration generate create_book`
- This will generate your migration folder and inside that you have to update the contents of `up.sql` and `down.sql`
<hr>

### 5. Run migration
`diesel migration run`<br>and now you can look up the DB and see that the table has been created
<hr>

### 6. Run your Rust application
`cargo run`
