use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let database_url = String::from("postgres://postgres:mypassword@localhost:5555?options=--search_path%3Dtest8");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}