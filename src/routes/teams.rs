use rocket::{post};
use rocket::serde::json::Json;
use crate::auth::Payload;
use crate::models::teams::{TeamDto, create_team};

#[post("/create-team", format = "json", data = "<data>")]
pub fn create_new_team(data: Json<TeamDto>, auth: Option<Payload>) -> String {
    match auth {
        Some(auth) => create_team(data.0, auth.user_id).team_id,
        None => String::from("None")
    }
}
