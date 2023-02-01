use diesel::{Insertable, Queryable};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    team::database::{
        insert::insert_member,
        select::{select_team_id_by_user_id, select_team_member},
    },
    utils::{db_connection::establish_connection, errors::ValidationErrors},
};

use super::dto::TeamMemberDto;
use crate::schema::teammember;

#[derive(Queryable, Insertable, Serialize, Deserialize, JsonSchema)]
#[table_name = "teammember"]
pub struct TeamMember {
    pub team_member_id: String,
    pub user_id: String,
    pub team_id: String,
    pub role_id: i32,
}

impl TeamMember {
    pub fn new(id: String, user_id: String, team_id: String, role_id: i32) -> TeamMember {
        TeamMember {
            team_member_id: id,
            user_id: user_id,
            team_id: team_id,
            role_id: role_id,
        }
    }

    pub fn add(
        team_member_dto: TeamMemberDto,
        team_id: Option<String>,
    ) -> Result<TeamMember, ValidationErrors> {
        let connection = &mut establish_connection();
        use uuid::Uuid;
        let id = Uuid::new_v4().to_string();
        let team_id = match team_id {
            Some(id) => id,
            None => return Err(ValidationErrors::NullValue("team id is null".to_string())),
        };
        let user_id = match team_member_dto.user_id {
            Some(data) => data,
            None => {
                return Err(ValidationErrors::InvalidValue(
                    "user id is null".to_string(),
                ))
            }
        };
        let team_member = select_team_member(connection, team_id.clone(), user_id.clone());
        match team_member {
            Some(_) => {
                return Err(ValidationErrors::InvalidValue(
                    "user already exists".to_string(),
                ))
            }
            None => (),
        }
        let role_id = match team_member_dto.role_id {
            Some(data) => data,
            None => return Err(ValidationErrors::NullValue("role id is null".to_string())),
        };
        let team_member = TeamMember::new(id, user_id, team_id, role_id);
        match insert_member(connection, team_member) {
            Ok(data) => Ok(data),
            Err(_) => {
                return Err(ValidationErrors::NullValue(
                    "Couldn't insert team member".to_string(),
                ))
            }
        }
    }

    pub fn find_all_teams(user_id: String) -> Vec<TeamMember> {
        let connection = &mut establish_connection();
        select_team_id_by_user_id(connection, user_id)
    }

    pub fn find_team_member(
        user_id: String,
        team_id: String,
    ) -> Result<TeamMember, ValidationErrors> {
        let connection = &mut establish_connection();
        let team_member = select_team_member(connection, team_id, user_id);
        match team_member {
            Some(data) => Ok(data),
            None => {
                return Err(ValidationErrors::NullValue(
                    "Couldn't find the user".to_string(),
                ))
            }
        }
    }
}
