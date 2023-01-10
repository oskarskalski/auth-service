use diesel::{Insertable, Queryable};
use crate::{schema::users, utils::{db_connection::establish_connection, errors::ValidationErrors}, users::database::{select::select_user, insert::insert_user}};

use super::user_dto::UserDto;

extern crate bcrypt;

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub e_mail: String,
    pub password: String,
    pub created_at: i64,
}

impl User {

    pub fn new(username: String, e_mail: String, password: String) -> User {
        use uuid::Uuid;
        let id = Uuid::new_v4().to_string();
        let date = chrono::offset::Utc::now().timestamp();
        User {
            user_id: id,
            username: username,
            e_mail: e_mail,
            password: password,
            created_at: date,
        }
    }

    pub fn create(user: UserDto) -> Result<User, ValidationErrors>{
        use bcrypt::{hash, DEFAULT_COST};

        let connection = &mut establish_connection();
        let username = match user.username {
            Some(data) => data,
            None => return Err(ValidationErrors::NullValue("Username is null".to_string())),
        };
        let find_user = select_user(connection, username.clone());
        match find_user {
            Some(_) => return Err(ValidationErrors::InvalidValue("User already exists".to_string())),
            None => (),
        };
        let user_passsword = match user.password {
            Some(data) => hash(data, DEFAULT_COST),
            None => return Err(ValidationErrors::NullValue("Password is empty".to_string())),
        };

        let password = match user_passsword {
            Ok(data) => data,
            Err(_) => return Err(ValidationErrors::InvalidValue("Couldn't validate password".to_string())),
        };
        let e_mail = match user.e_mail {
            Some(data) => data,
            None => return Err(ValidationErrors::NullValue("E-mail is empty".to_string())),
        };
        let new_user = User::new(username, e_mail, password);
        insert_user(connection, new_user.clone());
        return Ok(new_user);
    }

    pub fn verify_user(user: UserDto) -> Result<String, ValidationErrors> {
        use bcrypt::verify;
        let username = match user.username {
            Some(data) => data,
            None => return Err(ValidationErrors::NullValue("Username is null".to_string())),
        };
        let user_password = match user.password {
            Some(data) => data,
            None => return Err(ValidationErrors::NullValue("Password is null".to_string())),
        };
        let connection = &mut establish_connection();
        let find_user = select_user(connection, username);
        let find_user = match find_user {
            Some(data) => data,
            None => return Err(ValidationErrors::InvalidValue("User with the username not found".to_string())),
        };
        match verify(user_password, &find_user.password) {
            Ok(is_correct) => {
                if is_correct {
                    return Ok(find_user.user_id)
                }
                return Err(ValidationErrors::InvalidValue("The password is incorrect".to_string())) 
            },
            Err(_) => return Err(ValidationErrors::InvalidValue("Couldn't verify the password".to_string()))
        }
    }
}