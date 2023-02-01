use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::{
        self,
        openapi3::{MediaType, Responses, SecurityScheme, SecuritySchemeData, Object, SecurityRequirement},
    },
    response::OpenApiResponderInner,
    OpenApiError, request::RequestHeaderInput,
};

use rocket_okapi::request::OpenApiFromRequest;

use super::{response::{Response, ResponseDto}, request::RequestHolder};

impl<T: rocket_okapi::JsonSchema> OpenApiResponderInner for Response<'_, T> {
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use okapi::openapi3::RefOr;
        Ok(Responses {
            responses: okapi::map! {
                "400".to_owned() => RefOr::Object(bad_request_response::<T>(gen)),
                // "401".to_owned() => RefOr::Object(unauthorized_response(gen)),
            },
            ..Default::default()
        })
    }
}

pub fn bad_request_response<T: rocket_okapi::JsonSchema>(gen: &mut OpenApiGenerator) -> okapi::openapi3::Response {
    let schema = gen.json_schema::<ResponseDto<T>>();
    okapi::openapi3::Response {
        description: "\
        # 400 Bad Request\n\
        The request given is wrongly formatted or data was missing. \
        "
        .to_owned(),
        content: okapi::map! {
            "application/json".to_owned() => MediaType {
                schema: Some(schema),
                ..Default::default()
            }
        },
        ..Default::default()
    }
}

impl OpenApiFromRequest<'_> for RequestHolder {
    fn get_responses(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
    ) -> rocket_okapi::Result<rocket_okapi::okapi::openapi3::Responses> {
        Ok(rocket_okapi::okapi::openapi3::Responses::default())
    }

    fn from_request_input(
        gen: &mut rocket_okapi::gen::OpenApiGenerator,
        name: String,
        required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an Bearer token to access, token is: `mytoken`.".to_owned(),
            ),
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
