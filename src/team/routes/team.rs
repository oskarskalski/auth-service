use rocket::http::{Status};
use rocket::{post};
use rocket::serde::json::Json;
use crate::request::request::RequestHolder;
use crate::request::response::Response;
use crate::security::jwt::Jwt;
use crate::team::models::team::Team;
use crate::team::models::dto::TeamDto;
use crate::utils::errors::ValidationErrors;

#[post("/create-team", format = "json", data = "<data>")]
pub fn create_new_team(data: Json<TeamDto>, request_holder: RequestHolder) -> Response<'static, String> {
    let mut response = Response::map(request_holder.clone());
    let payload = request_holder.payload.unwrap();
    match Team::create(data.0, payload.user_id.clone()) {
        Ok(team) => {
            response.update_body_and_status(String::from("Team has been created successfully"), Status::Created);
            response = add_jwt(team.jwt(payload.user_id), response);
        },
        Err(err) => response.update_error(String::from(err.message()), Status::BadRequest)
    };
    response
}

#[get("/choose-team/<team_id>")]
pub fn get_access_to_team(team_id: String, request_holder: RequestHolder) -> Response<'static, String>{
    let mut response: Response<String> = Response::map(request_holder.clone());
    let payload = request_holder.payload.unwrap();
    response.update_body(String::from("Managed to log into the team"));
    add_jwt(Jwt::create(payload.user_id, Some(team_id), None), response)
}

fn add_jwt(jwt: Result<Jwt, ValidationErrors>, mut response: Response<String>) -> Response<String> {
    match jwt {
        Ok(data) => {
            response.add_header(String::from("Authorization"), String::from("Bearer ") + &data.token.unwrap());
        },
        Err(err) => response.update_body_and_status(String::from("Couldn't log into created team. Try again..."), Status::InternalServerError),
    };
    response
}