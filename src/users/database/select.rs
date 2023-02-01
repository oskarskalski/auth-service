use crate::schema::users;
use crate::users::models::user::User;
use crate::utils::db_connection::DbConnection;

pub struct SelectOperations {}

impl SelectOperations {
    pub fn select_user(e_mail: String) -> Option<User> {
        let connection_pool = DbConnection::new();
        let connection = &mut connection_pool.get();
        use diesel::prelude::*;
        users::table
            .filter(users::e_mail.eq(&e_mail))
            .get_result::<User>(connection)
            .ok()
    }
}

