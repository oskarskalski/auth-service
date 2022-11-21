use crate::database::{teams::{Team, insert_team, select_team_ids}, connection::establish_connection};

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
        creator_id: user_id,
        description: description,
        team_name: name
    };

    insert_team(connection, new_team)
}

pub fn get_team_ids(user_id: String) -> Vec<String> {
    let connection = &mut establish_connection();
    let teams = select_team_ids(connection, user_id);
    let result = match teams {
        Some(data) => data.into_iter().map(|f| (f.team_id)).collect(),
        None => panic!("Error occured while mapping reuslts for teams ids")
    };
    return result;
}