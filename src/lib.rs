
#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;
mod users;
mod database;
mod utils;
mod routes;
mod security;
use dotenv::dotenv;
use routes::roles::role_ids;
use crate::routes::roles::add_team_role;
use crate::routes::teams::create_new_team;
use crate::routes::roles::team_roles;
use crate::users::routes::sign_user::{sign_in_user, sign_up_user};

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount("/auth", routes![sign_in_user, sign_up_user, create_new_team, role_ids, team_roles, add_team_role])
        .register("/", catchers![not_found, unauthorized, forbidden])
}

//routes::users::sign_in_user, routes::users::sign_up_user
#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({
        "status": "error",
        "reason": "User is not authorized"
    })
}
#[catch(403)]
fn forbidden() -> Value {
    json!({
        "status": "error",
        "reason": "Resource access is forbidden"
    })
}