use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamDto{
    pub team_name: Option<String>,
    pub description: Option<String>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TeamMemberDto {
    pub user_id: Option<String>,
    pub role_id: Option<i32>
}

impl TeamMemberDto {
    pub fn new(user_id: Option<String>, role_id: Option<i32>) -> TeamMemberDto{
        TeamMemberDto { user_id: user_id, role_id: role_id }
    }
}