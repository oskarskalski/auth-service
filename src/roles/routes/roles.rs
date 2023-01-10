use rocket::serde::json::Json;

use crate::roles::models::roles::SystemRole;


#[get("/roles", format = "json")]
pub fn role_ids() -> Json<Vec<SystemRole>>{
    Json(SystemRole::get_all())
}

