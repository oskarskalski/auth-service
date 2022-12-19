use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    pub username: Option<String>,
    pub password: Option<String>,
    pub e_mail: Option<String>,
}
