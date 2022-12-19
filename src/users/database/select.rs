use diesel::pg::PgConnection;
use crate::schema::users;
use crate::users::models::user::User;

pub fn select_user(conn: &mut PgConnection, user: String) -> Option<User> {
    use diesel::prelude::*;
    users::table
        .filter(users::username.eq(&user))
        .get_result::<User>(conn)
        .ok()
}
