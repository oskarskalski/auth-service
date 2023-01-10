use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

use crate::config;

use super::jwt::Jwt;
use super::payload::Payload;


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Payload {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Payload, Self::Error> {
        if let Some(auth) = extract_auth_from_request(req) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request: &Request) -> Option<Payload> {
    request
        .headers()
        .get_one("Authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| Some(Jwt::verify(token)))
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}
