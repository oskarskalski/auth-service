use rocket::serde::json::Json;

use crate::team::models::dto::TeamMemberDto;
use crate::{security::payload::Payload, team::models::team_member::TeamMember};

#[get("/get-teams")]
pub fn get_user_teams(auth: Option<Payload>) -> Json<Vec<TeamMember>> {
    match auth {
        Some(auth) => Json(TeamMember::find_all_teams(auth.user_id)),
        None => panic!("Couldn't verify JWT"),
    }
}

#[post("/add-team-member", format = "json", data = "<data>")]
pub fn add_team_member(data: Json<TeamMemberDto>, auth: Payload) -> String {
    let result = TeamMember::add(data.0, auth.team_id);
    match result {
        Ok(data) => data.team_member_id,
        Err(_) => String::from("_"),
    }
}
