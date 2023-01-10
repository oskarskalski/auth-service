use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamRolesDto{
    role_name: String,
    role_ids: Vec<i32>
}
