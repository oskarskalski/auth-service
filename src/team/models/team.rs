use diesel::{Insertable, Queryable};

use crate::{
    schema::teams,
    security::jwt::Jwt,
    team::database::{insert::insert_team, select::select_team_by_name},
    utils::{db_connection::establish_connection, errors::ValidationErrors},
};

use super::{
    dto::{TeamDto, TeamMemberDto},
    team_member::TeamMember,
};

#[derive(Queryable, Insertable)]
#[diesel(table_name = teams)]
pub struct Team {
    pub team_id: String,
    pub team_name: String,
    pub description: String,
}

impl Team {
    pub fn new(description: String, team_name: String) -> Team {
        use uuid::Uuid;
        let id = Uuid::new_v4().to_string();
        Team {
            team_id: id,
            description: description,
            team_name: team_name,
        }
    }

    pub fn create(team: TeamDto, user_id: String) -> Result<Team, ValidationErrors> {
        let connection = &mut establish_connection();
        let name = match team.team_name {
            Some(name) => name,
            None => return Err(ValidationErrors::NullValue("Name is null".to_string())),
        };
        match select_team_by_name(connection, name.clone()) {
            Some(_) => {
                return Err(ValidationErrors::InvalidValue(
                    "Team with this name already exists".to_string(),
                ))
            }
            None => (),
        }
        let description: String = team.description.unwrap_or(String::from(""));
        let new_team = Self::new(description, name);
        let team = insert_team(connection, new_team);
        let team_member = TeamMemberDto::new(Some(user_id), Some(1));
        match TeamMember::add(team_member, Some(team.team_id.to_owned())) {
            Ok(data) => (),
            Err(err) => return Err(err),
        }
        return Ok(team);
    }

    pub fn jwt(self, user_id: String) -> Result<Jwt, ValidationErrors> {
        Jwt::create(user_id, Some(self.team_id), None)
    }
}
