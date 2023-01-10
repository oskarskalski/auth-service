use diesel::pg::PgConnection;
use crate::schema::users;
use crate::users::models::user::User;
use diesel::RunQueryDsl;

pub fn insert_user(conn: &mut PgConnection, new_user: User) -> User {
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .unwrap();
    new_user
}

