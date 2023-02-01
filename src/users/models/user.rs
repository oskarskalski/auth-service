use crate::{
    schema::users,
    users::database::{insert::InsertOperations, select::SelectOperations},
    utils::errors::ValidationErrors,
};
use diesel::{Insertable, Queryable};

use super::user_dto::UserDto;

extern crate bcrypt;

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub e_mail: String,
    pub password: String,
    pub created_at: i64,
}

impl User {
    pub fn new(user: UserDto) -> Result<Self, ValidationErrors> {
        use bcrypt::{hash, DEFAULT_COST};
        use uuid::Uuid;
        let id = Uuid::new_v4().to_string();
        let date = chrono::offset::Utc::now().timestamp();
        let first_name = match user.first_name {
            Some(data) => data,
            None => {
                return Err(ValidationErrors::NullValue(
                    "First name is null".to_string(),
                ))
            }
        };
        let last_name = match user.last_name {
            Some(data) => data,
            None => return Err(ValidationErrors::NullValue("Last name is null".to_string())),
        };
        let e_mail = match user.e_mail {
            Some(data) => data,
            None => return Err(ValidationErrors::NullValue("E-mail is empty".to_string())),
        };
        let user_passsword = match user.password {
            Some(data) => hash(data, DEFAULT_COST),
            None => return Err(ValidationErrors::NullValue("Password is empty".to_string())),
        };

        let password = match user_passsword {
            Ok(data) => data,
            Err(_) => {
                return Err(ValidationErrors::InvalidValue(
                    "Couldn't validate password".to_string(),
                ))
            }
        };
        Ok(
            User {
                user_id: id,
                e_mail: e_mail,
                first_name: first_name,
                last_name: last_name,
                password: password,
                created_at: date,
            }
        )
    }

    pub fn create(&self) -> Result<i32, ValidationErrors> {
        let find_user = self.get_user_by_email();
        match find_user {
            Some(_) => {
                return Err(ValidationErrors::InvalidValue(
                    "User already exists".to_string(),
                ))
            }
            None => (),
        };
        self.insert_user();
        return Ok(1);
    }

    pub fn verify_user(&self, user: UserDto) -> Result<String, ValidationErrors> {
        use bcrypt::verify;
        let e_mail = match user.e_mail {
            Some(data) => data,
            None => return Err(ValidationErrors::NullValue("E-mail is null".to_string())),
        };
        let user_password = match user.password {
            Some(data) => data,
            None => return Err(ValidationErrors::NullValue("Password is null".to_string())),
        };
        let find_user = self.get_user_by_email();
        let find_user = match find_user {
            Some(data) => data,
            None => {
                return Err(ValidationErrors::InvalidValue(
                    "User with the username not found".to_string(),
                ))
            }
        };
        match verify(user_password, &find_user.password) {
            Ok(is_correct) => {
                if is_correct {
                    return Ok(find_user.user_id);
                }
                return Err(ValidationErrors::InvalidValue(
                    "The password is incorrect".to_string(),
                ));
            }
            Err(_) => {
                return Err(ValidationErrors::InvalidValue(
                    "Couldn't verify the password".to_string(),
                ))
            }
        }
    }

    pub fn get_user_by_email(&self) -> Option<User> {
        SelectOperations::select_user(self.e_mail.clone())
    }

    pub fn insert_user(&self) -> User {
        InsertOperations::insert_user(self.clone())
    }
}
