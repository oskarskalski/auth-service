use crate::request::request::RequestHolder;
use crate::request::response::Response;
use crate::security::jwt::Jwt;
use crate::users::models::user::User;
use crate::users::models::user_dto::UserDto;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;

#[post("/sign-in", format = "json", data = "<data>")]
pub fn sign_in_user(
    data: Json<UserDto>,
    request_holder: RequestHolder,
) -> Response<'static, String> {
    let user = data.0;
    let user = User::verify_user(user);
    let jwt = match user {
        Ok(id) => Jwt::create(id, None, None),
        Err(err) => Err(err),
    };
    let mut response = Response::map(request_holder);
    match jwt {
        Ok(data) => {
            response.update_body(String::from("User has been validated sucessfully"));
            response.add_header(String::from("Authorization"), String::from("Bearer ") + &data.token.unwrap());
        }
        Err(err) => response.update_error(String::from(err.message()), Status::BadRequest),
    };
    response
}

#[post("/sign-up", format = "json", data = "<data>")]
pub fn sign_up_user(
    data: Json<UserDto>,
    request_holder: RequestHolder,
) -> Response<'static, String> {
    let mut response = Response::map(request_holder);
    match User::create(data.0) {
        Ok(_) => response.update_body_and_status(String::from("User has been created successfully."), Status::Created),
        Err(err) => response.update_error(String::from(err.message()), Status::BadRequest)
    }
    response
}
