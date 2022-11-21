use serde::{Deserialize, Serialize};
extern crate bcrypt;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::{auth::create_jwt, database::{connection::establish_connection, users::{select_user, User, insert_user}}};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    pub username: Option<String>,
    pub password: Option<String>,
    pub e_mail: Option<String>,
}

pub fn verify_user(user: UserDto) -> String {
    let connection = &mut establish_connection();
    let username = match user.username {
        Some(data) => data,
        None => String::from("eo"),
    };

    let find_user = select_user(connection, username);
    let find_user = match find_user {
        Some(data) => data,
        None => panic!("Error occured while finding user"),
    };

    let user_password = match user.password {
        Some(data) => data,
        None => panic!("Error occured while checking the password"),
    };

    let verify_password = verify(user_password, &find_user.password);

    let result = match verify_password {
        Ok(data) => {
            if data {
                create_jwt(&find_user.user_id)
            } else {
                panic!("Error occured while creating jwt")
            }
        }
        Err(_) => panic!("Error occured while veryfing password"),
    };
    return result;
}

pub fn create_user(user: UserDto) {
    let connection = &mut establish_connection();

    use uuid::Uuid;
    let id = Uuid::new_v4().to_string();
    let date = chrono::offset::Utc::now().timestamp();

    let username = match user.username {
        Some(data) => data,
        None => String::from("_"),
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
