extern crate bcrypt;
use bcrypt::{hash, DEFAULT_COST};

use crate::{users::{models::{user_dto::UserDto, user::User}, database::{select::select_user, insert::insert_user}}, utils::db_connection::establish_connection};

pub fn create_user(user: UserDto) {
    let connection = &mut establish_connection();

    use uuid::Uuid;
    let id = Uuid::new_v4().to_string();
    let date = chrono::offset::Utc::now().timestamp();

    let username = match user.username {
        Some(data) => data,
        None => String::from("_"),
    };

    let find_user = select_user(connection, username.clone());
    match find_user {
        Some(data) => panic!("User already exists"),
        None => String::from(""),
    };

    let user_passsword = match user.password {
        Some(data) => hash(data, DEFAULT_COST),
        None => Ok(String::from("_")),
    };

    let pass = match user_passsword {
        Ok(data) => data,
        Err(e) => String::from("_"),
    };

    let e_mail = match user.e_mail {
        Some(data) => data,
        None => String::from("_"),
    };

    let new_user = User {
        user_id: id,
        username: username,
        e_mail: e_mail,
        password: pass,
        created_at: date,
    };

    insert_user(connection, new_user);
}
