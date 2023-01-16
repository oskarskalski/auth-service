use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::config::Config;
pub fn establish_connection() -> PgConnection {
    // let database_url = env::var("DATABASE_URL").expect("Couldn't find database url");
    let config = Config::new(None);
    let database_url = String::from("postgres://postgres:mypassword@localhost:5555");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}