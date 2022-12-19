use rocket::{post};
use rocket::serde::json::Json;
use crate::database::team_role::{TeamRoles, TeamRole};
use crate::models::team_roles::{TeamRolesDto, create_team_role, get_role_ids, get_roles, get_team_roles};
use crate::security::auth::Payload;

#[post("/create-team-role", format = "json", data = "<data>")]
pub fn add_team_role(data: Json<TeamRolesDto>, auth: Option<Payload>) {
    create_team_role(data, auth);
}


#[get("/roles", format = "json")]
pub fn role_ids(auth: Option<Payload>) -> Json<Vec<TeamRoles>>{
    Json(get_roles())
}



#[get("/team/roles", format = "json")]
pub fn team_roles(auth: Option<Payload>) -> Json<Vec<TeamRole>>{
    let roles = match auth {
        Some(payload) => Json(get_team_roles(payload.team_id)),
        None => panic!("No JWT inside the request")
    };
    return roles;
}
