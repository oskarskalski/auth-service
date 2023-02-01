use crate::schema::users;
use crate::users::models::user::User;
use crate::utils::db_connection::DbConnection;
use diesel::RunQueryDsl;

pub struct InsertOperations {}

impl InsertOperations {
    pub fn insert_user(new_user: User) -> User {
        let connection_pool = DbConnection::new();
        let connection = &mut connection_pool.get();
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)
            .unwrap();
        new_user
    }
}
