use serde::{Serialize, Deserialize};
use crate::utils::{responders::build_response, errors::ValidationErrors};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha384;
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use super::payload::Payload;
use rocket::{response::{Responder, self}, Request, http::Status};


#[derive(Serialize, Deserialize, Default)]
pub struct Jwt{
    pub token: Option<String>,
}

impl<'r> Responder<'r, 'static> for Jwt {

    fn respond_to(self, _request: &Request) -> response::Result<'static> {
        use rocket::http::Header;

        match self.token {
            Some(token) => {
                let jwt_prefix: String = String::from("Bearer");
                let header: String = [jwt_prefix, token].join(" ");
                let header: Header = Header::new("Authorization", header);
                Ok(build_response(String::from("{}"), vec![header], Status::Ok))
            }, 
            None =>  Ok(build_response(String::from("{}"), vec![], Status::Forbidden))
        }
    }
}

impl Jwt {
    pub fn new(value: String) -> Self {
        Self {
            token: Some(value),
        }
    }
    
    fn sign(payload: Payload) -> String {
        let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
        let header: Header = Header {
            algorithm: AlgorithmType::Hs384,
            ..Default::default()
        };
        let token: String = Token::new(header, payload)
            .sign_with_key(&key)
            .unwrap()
            .as_str()
            .to_string();
        return token;
    }

    pub fn create(user_id: String, team_id: Option<String>, group_id: Option<String>) -> Result<Jwt, ValidationErrors> {
        let payload = Payload::create(user_id, team_id, group_id);
        match payload {
            Ok(data) => Ok(Jwt::new(Self::sign(data))),
            Err(err) => return Err(err)
        }
    }

    pub fn verify(jwt: &str) -> Payload {
        let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
        match jwt.verify_with_key(&key) {
            Ok(r) => r,
            Err(_) => panic!("User verification has failed!"),
        }
    }
}