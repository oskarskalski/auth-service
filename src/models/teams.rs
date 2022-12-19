use crate::{database::{teams::{Team, insert_team, select_team}, connection::establish_connection, team_members::insert_member, team_role::TeamRole}, models::{team_members::{self, TeamMemberDto, add_team_member}, team_roles::create_team_lead_role}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamDto{
    pub team_name: Option<String>,
    pub description: Option<String>
}

pub fn create_team(team: TeamDto, user_id: String) -> Team{
    let connection = &mut establish_connection();
    use uuid::Uuid;

    let name = match team.team_name {
        Some(name) => name,
        None => String::from("_")
    };
    let description = match team.description {
        Some(description) => description,
        None => String::from("_")
    };
    let id = Uuid::new_v4().to_string();
    let new_team = Team {
        team_id: id,
        description,
        team_name: name
    };
    let team = insert_team(connection, new_team);
    let role_id: String = create_team_lead_role(team.team_id.to_owned());
    let team_member = TeamMemberDto {
        user_id: Some(user_id),
        role_id: Some(role_id)
    };
    add_team_member(team_member, team.team_id.to_owned());
    return team;
}

pub fn get_team_ids(team_id: String) -> Vec<String> {
    let connection = &mut establish_connection();
    let teams = select_team(connection, team_id);
    let result = match teams {
        // Some(data) => data.into_iter().map(|f| (f.team_id)).collect(),
        Some(data) => data.team_id,
        None => panic!("Error occured while mapping result for teams ids")
    };
    return vec![result];
}
