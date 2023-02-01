#[macro_use]
extern crate rocket;
use std::env;

use config::Config;
use rocket::serde::json::{json, Value};
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::make_rapidoc;
use rocket_okapi::rapidoc::GeneralConfig;
use rocket_okapi::rapidoc::HideShowConfig;
use rocket_okapi::rapidoc::RapiDocConfig;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::make_swagger_ui;
use rocket_okapi::swagger_ui::SwaggerUIConfig;

mod config;
mod migration;
mod rest;
mod roles;
mod schema;
mod tests;
mod security;
mod team;
mod users;
mod utils;
use crate::roles::routes::roles::role_ids;
use crate::team::routes::team::create_new_team;
use crate::team::routes::team::get_access_to_team;
use crate::users::routes::user_auth::{login, register};
use dotenv::dotenv;
use utils::db_connection::establish_connection;
use crate::roles::routes::roles::okapi_add_operation_for_role_ids_;
use crate::team::routes::team::okapi_add_operation_for_create_new_team_;
use crate::team::routes::team::okapi_add_operation_for_get_access_to_team_;
use crate::team::routes::team_members::okapi_add_operation_for_add_team_member_;
use crate::team::routes::team_members::okapi_add_operation_for_get_user_teams_;
use crate::team::routes::team_members::{add_team_member, get_user_teams};
use crate::users::routes::user_auth::okapi_add_operation_for_login_;
use crate::users::routes::user_auth::okapi_add_operation_for_register_;

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
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/auth/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "/auth/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .mount(
            "/auth",
            openapi_get_routes![
                login,
                register,
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
