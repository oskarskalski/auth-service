use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::CONFIG;

pub fn establish_connection() -> PgConnection {
    let database_url = create_database_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_database_url() -> String{
    let config = &CONFIG;
    let mut db_url: String = String::from("postgres://");
    db_url.push_str(&config.username);
    db_url.push_str(":");
    db_url.push_str(&config.password);
    db_url.push_str("@");
    db_url.push_str(&config.hostname);
    db_url
}