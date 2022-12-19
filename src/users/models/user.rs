use diesel::{Insertable, Queryable};
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
