use rocket::http::Status;
use rocket::serde::json::Json;

use crate::request::request::RequestHolder;
use crate::request::response::Response;
use crate::team::models::dto::TeamMemberDto;
use crate::team::models::team_member::TeamMember;

#[get("/get-teams")]
pub fn get_user_teams(request_holder: RequestHolder) -> Response<'static, Vec<TeamMember>> {
    let mut response = Response::map(request_holder.clone());
    let payload = request_holder.payload.unwrap();
    let team_members = TeamMember::find_all_teams(payload.user_id);
    response.update_body(team_members);
    response
}

#[post("/add-team-member", format = "json", data = "<data>")]
pub fn add_team_member(
    data: Json<TeamMemberDto>,
    request_holder: RequestHolder,
) -> Response<'static, String> {
    let mut response = Response::map(request_holder.clone());
    let payload = request_holder.payload.unwrap();
    let result = TeamMember::add(data.0, payload.team_id);
    match result {
        Ok(_) => {
            response.update_body_and_status("User has been added succesfully".to_string(), Status::Created);
        }, 
        Err(err) => response.update_error(err.message().to_string(), Status::BadRequest),
    };
    response
}
