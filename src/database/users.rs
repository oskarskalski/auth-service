use diesel::pg::PgConnection;

use diesel::{Insertable, Queryable, RunQueryDsl};

use crate::schema::users;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub e_mail: String,
    pub password: String,
    pub created_at: i64,
}

pub fn insert_user(conn: &mut PgConnection, new_user: User) -> User {
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .unwrap();
    new_user
}

pub fn select_user(conn: &mut PgConnection, user: String) -> Option<User> {
    use diesel::prelude::*;
    users::table
        .filter(users::username.eq(&user))
        .get_result::<User>(conn)
        .ok()
}
