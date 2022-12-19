use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

use crate::config;
use crate::models::team_roles::get_team_id_by_user_id;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub timestamp: u64,
    pub user_id: String,
    pub team_id: String, 
    pub expire_time: u64,
}

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
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| Some(verify_jwt(token)))
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

pub fn create_jwt(user_id: &str) -> String {
    let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
    let header: Header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let payload = create_payload(user_id);
    let token: String = Token::new(header, payload)
        .sign_with_key(&key)
        .unwrap()
        .as_str()
        .to_string();
    return token;
}

pub fn create_payload(user_id: &str) -> Payload {
    let start: SystemTime = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp: u64 = since_the_epoch.as_secs();
    let expire_time: u64 = since_the_epoch.as_secs() + 86400;
    let team_id: String = match get_team_id(user_id.to_string()) {
        Some(data) => data,
        None => String::from("")
    };
    let payload: Payload = Payload {
        timestamp: timestamp,
        user_id: user_id.to_string(),
        expire_time: expire_time,
        team_id: team_id,
    };
    return payload;
}

pub fn verify_jwt(jwt: &str) -> Payload {
    let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
    match jwt.verify_with_key(&key) {
        Ok(r) => r,
        Err(_) => panic!("User verification has failed!"),
    }
}

pub fn get_team_id(user_id: String) -> Option<String>{
    get_team_id_by_user_id(user_id)
}