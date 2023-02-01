use rocket::response::Responder;
use rocket::{
    http::{Header, Status},
    response, Request,
};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::utils::responders::build_response;

use super::request::RequestHolder;

#[derive(Default)]
pub struct Response<'a, T> {
    pub headers: Vec<Header<'a>>,
    pub body: Option<T>,
    pub error: Option<String>,
    pub response_time: u64,
    pub status: Status,
    pub path: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ResponseDto<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub response_time: u64,
    pub path: String,
}

impl<'a, T> Responder<'a, 'a> for Response<'a, T>
where
    T: Serialize,
{
    fn respond_to(self, _request: &Request) -> response::Result<'a> {
        let response_dto = ResponseDto {
            body: self.body,
            response_time: self.response_time,
            error: self.error,
            path: self.path,
        };
        let reponse_dto = serde_json::to_string(&response_dto).unwrap();
        Ok(build_response(reponse_dto, self.headers, self.status))
    }
}

impl<'a, T> Response<'a, T> {
    pub fn map(request: RequestHolder) -> Response<'a, T> {
        Response {
            headers: vec![],
            response_time: request.timestamp,
            path: request.path,
            status: Status::Ok,
            body: None,
            error: None,
        }
    }

    pub fn update_error(&mut self, error: String, status: Status) {
        self.error = Some(error);
        self.status = status;
    }

    pub fn update_body_and_status(&mut self, body: T, status: Status) {
        self.body = Some(body);
        self.status = status;
    }

    pub fn update_body(&mut self, body: T) {
        self.body = Some(body);
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.push(Header::new(key, value));
    }
}
