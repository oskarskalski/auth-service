use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::{rest::{response::Response, request::RequestHolder}, roles::models::roles::SystemRole};

#[openapi]
#[get("/roles", format = "json")]
pub fn role_ids(request_holder: RequestHolder) -> Response<'static, Vec<SystemRole>> {
    let mut response = Response::map(request_holder.clone());
    response.update_body(SystemRole::get_all());
    response
}
