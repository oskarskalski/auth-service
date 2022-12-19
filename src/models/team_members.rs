use serde::{Deserialize, Serialize};

use crate::{database::{connection::establish_connection, team_members::{insert_member, TeamMember}}};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamMemberDto {
    pub user_id: Option<String>,
    pub role_id: Option<String>
}

pub fn add_team_member(team_member_dto: TeamMemberDto, team_id: String) -> TeamMember{
    let connection = &mut establish_connection();
    use uuid::Uuid;
    let id = Uuid::new_v4().to_string();
    let user_id =  match team_member_dto.user_id {
        Some(data) => data,
        None => String::from("_")
    };
    let role_id = match  team_member_dto.role_id {
        Some(data) => data,
        None => String::from("")
    };
    let team_member = TeamMember {
        team_member_id: id,
        user_id:  user_id,
        team_id: team_id,
        role_id: role_id,
    };
    insert_member(connection, team_member)
}
