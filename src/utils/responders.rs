use rocket::response::{Response};
use rocket::http::{ContentType, Status, Header};
use std::io::Cursor;

pub fn build_response(body: String, headers: Vec<Header>, status: Status) -> Response {
    let mut binding = Response::build();
    let response = binding
    .header(ContentType::JSON)
    .status(status)
    .sized_body(body.len(), Cursor::new(body));
    for header in headers {
        response.header(header);
    }
    response.finalize()
}