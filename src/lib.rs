#[macro_use]
extern crate rocket;
use std::env;

use config::Config;
use rocket::serde::json::{json, Value};

mod config;
mod migration;
mod request;
mod roles;
mod schema;
mod security;
mod team;
mod users;
mod utils;
use crate::roles::routes::roles::role_ids;
use crate::team::routes::team::create_new_team;
use crate::team::routes::team::get_access_to_team;
use crate::users::routes::sign_user::{sign_in_user, sign_up_user};
use dotenv::dotenv;
use utils::db_connection::establish_connection;

use crate::team::routes::team_members::{add_team_member, get_user_teams};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Config = {
        let args: Vec<String> = env::args().collect();
        let config_path = if args.len() > 1 {
            Some(args[1].as_str())
        } else {
            None
        };
        Config::new(config_path)
    };
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    migration::run_migration(&mut establish_connection());
    use rocket::config::Config as RocketConfig;

    rocket::custom(RocketConfig::figment())
        .mount(
            "/auth",
            routes![
                sign_in_user,
                sign_up_user,
                create_new_team,
                role_ids,
                add_team_member,
                get_user_teams,
                get_access_to_team
            ],
        )
        .register("/", catchers![not_found, unauthorized, forbidden])
}

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
