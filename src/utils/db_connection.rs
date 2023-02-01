use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::PooledConnection;
use crate::CONFIG;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub fn establish_connection() -> PgConnection {
    let database_url = create_database_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct DbConnection {
    pub pool: Pool<ConnectionManager<PgConnection>>
}

impl DbConnection {
    pub fn new() -> Self{
        let manager = ConnectionManager::<PgConnection>::new(create_database_url());
        let connection_pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");
        DbConnection { pool: connection_pool }
    }
    pub fn get(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.clone().get().unwrap()
    }
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
