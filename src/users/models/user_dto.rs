use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct UserDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub e_mail: Option<String>,
}
