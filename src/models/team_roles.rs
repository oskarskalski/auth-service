use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{database::{connection::establish_connection, team_members::{insert_member, TeamMember, select_team_id_by_user_id}, team_role::{select_role_ids, select_roles, TeamRoleMap, TeamRole, insert_team_role, save_role_relation, TeamRoles, select_team_roles}}, security::auth::Payload};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamRolesDto{
    role_name: String,
    role_ids: Vec<i32>
}

pub fn create_team_lead_role(team_id: String) -> String{
    let id = Uuid::new_v4().to_string();
    let connection = &mut establish_connection();
    let team_role = TeamRole {
        team_id: team_id,
        team_role_id: id,
        role_name: String::from("TEAM_LEAD"),
    };
    let team_role_id = insert_team_role(connection, team_role);
    let ids: Vec<i32> = get_role_ids();
    for role_id in ids {
        let team_member_map = TeamRoleMap {
            id: None,
            role_id: role_id,
            team_role_id: team_role_id.clone()
        };
        save_role_relation(connection, team_member_map);
    }
    return team_role_id;
}

pub fn get_role_ids() -> Vec<i32> {
    let connection = &mut establish_connection();
    select_role_ids(connection)
}

pub fn get_roles() -> Vec<TeamRoles> {
    let connection = &mut establish_connection();
    select_roles(connection)
}

pub fn create_team_role(data: Json<TeamRolesDto>, auth: Option<Payload>){
    let connection = &mut establish_connection();
    let team_id = match auth {
        Some(data) => data.team_id,
        None => panic!("Couldn't find team id inside the request")
    };
    let data = data.0;
    let id = Uuid::new_v4().to_string();
    let team_role = TeamRole{
        team_role_id: id.clone(),
        role_name: data.role_name,
        team_id: team_id,
    };
    insert_team_role(connection, team_role);
    for role_id in data.role_ids {
        let team_member_map = TeamRoleMap {
            id: None,
            role_id: role_id,
            team_role_id: id.clone()
        };
        save_role_relation(connection, team_member_map);
    }
}

pub fn get_team_roles(team_id: String) -> Vec<TeamRole>{
    let connection = &mut establish_connection();
    select_team_roles(connection, team_id)
}

pub fn get_team_id_by_user_id(user_id: String) -> Option<String>{
    let connection = &mut establish_connection();
    select_team_id_by_user_id(connection, user_id)
}

// pub fn get_user_role(team_id: String, user_id: String) -> Option<String> {
    
// }