use rocket::post;
use rocket::serde::json::Json;
use crate::users::models::user_dto::UserDto;
use crate::users::service::create_user::create_user;
use crate::users::service::insert_user::verify_user;

#[post("/sign-in", format = "json", data = "<data>")]
pub fn sign_in_user(data: Json<UserDto>) -> String {
    let token = verify_user(data.0);
    return token;
}

#[post("/sign-up", format = "json", data = "<data>")]
pub fn sign_up_user(data: Json<UserDto>) -> String {
    create_user(data.0);
    return String::from("hello");
}
