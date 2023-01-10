use std::time::{SystemTime, UNIX_EPOCH};

use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

use crate::config;

use crate::security::jwt::Jwt;
use crate::security::payload::Payload;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestHolder {
    pub payload: Option<Payload>,
    pub request_id: String,
    pub timestamp: u64,
    pub path: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHolder {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<RequestHolder, Self::Error> {
        use uuid::Uuid;
        let skipped_paths: Vec<String> =
            vec!["/auth/sign-in".to_string(), "/auth/sign-up".to_string()];
        let path = req.uri().path().to_string();
        let request_id = extract_request_id_from_request(req);
        let request_id = match request_id {
            Some(data) => data,
            None => Uuid::new_v4().to_string(),
        };
        let start: SystemTime = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp: u64 = since_the_epoch.as_secs();
        let mut request_holder = RequestHolder {
            request_id: request_id,
            timestamp: timestamp,
            payload: None,
            path: path.clone(),
        };
        if !skipped_paths.contains(&path) {
            if let Some(payload) = extract_auth_from_request(req) {
                request_holder.payload = Some(payload);
                return Outcome::Success(request_holder);
            } else {
                return Outcome::Failure((Status::Forbidden, ()));
            }
        }
        Outcome::Success(request_holder)
    }
}

fn extract_auth_from_request(request: &Request) -> Option<Payload> {
    request
        .headers()
        .get_one("Authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| Some(Jwt::verify(token)))
}

fn extract_request_id_from_request(request: &Request) -> Option<String> {
    let request_id = request.headers().get_one("request-id");
    match request_id {
        Some(data) => Some(data.to_string()),
        None => None,
    }
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}
