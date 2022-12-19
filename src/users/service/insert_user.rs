extern crate bcrypt;
use bcrypt::verify;

use crate::{security::auth::create_jwt, database::{connection::establish_connection}, users::{models::user_dto::UserDto, database::select::select_user}};

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
