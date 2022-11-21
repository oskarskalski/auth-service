
#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;
mod database;
mod auth;
mod routes;
use dotenv::dotenv;

use crate::routes::teams::create_new_team;

use crate::routes::users::{sign_in_user, sign_up_user};

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount("/auth", routes![sign_in_user, sign_up_user, create_new_team])
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