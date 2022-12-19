use rocket::{post};
use rocket::serde::json::Json;
use crate::models::teams::{TeamDto, create_team};
use crate::security::auth::Payload;

#[post("/create-team", format = "json", data = "<data>")]
pub fn create_new_team(data: Json<TeamDto>, auth: Option<Payload>) -> String {
    match auth {
        Some(auth) => create_team(data.0, auth.user_id).team_id,
        None => String::from("None")
    }
}


#[post("/add-team-member", format = "json", data = "<data>")]
pub fn add_team_member(data: Json<TeamDto>, auth: Option<Payload>) -> String {
    match auth {
        Some(auth) => create_team(data.0, auth.user_id).team_id,
        None => String::from("None")
    }
}

