use rocket::FromForm;
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::openapi3::{Parameter, ParameterValue},
    request::OpenApiFromForm,
};

#[derive(FromForm, Debug)]
pub struct VerificationRequest {
    pub hub: Hub,
}

#[derive(FromForm, Debug)]
pub struct Hub {
    pub mode: String,
    pub challenge: String,
    pub verify_token: String,
}

impl<'v> OpenApiFromForm<'v> for VerificationRequest {
    fn form_multi_parameter(
        gen: &mut OpenApiGenerator,
        _name: String,
        required: bool,
    ) -> rocket_okapi::Result<Vec<Parameter>> {
        Ok(vec![
            Parameter {
                name: "hub.mode".to_owned(),
                location: "query".to_owned(),
                description: Some("Subscription mode".to_owned()),
                required,
                deprecated: false,
                allow_empty_value: false,
                value: ParameterValue::Schema {
                    style: None,
                    explode: None,
                    allow_reserved: false,
                    schema: gen.json_schema::<String>(),
                    example: Some(serde_json::json!("subscribe")),
                    examples: None,
                },
                extensions: Default::default(),
            },
            Parameter {
                name: "hub.challenge".to_owned(),
                location: "query".to_owned(),
                description: Some("Challenge code".to_owned()),
                required,
                deprecated: false,
                allow_empty_value: false,
                value: ParameterValue::Schema {
                    style: None,
                    explode: None,
                    allow_reserved: false,
                    schema: gen.json_schema::<String>(),
                    example: Some(serde_json::json!("1158201444")),
                    examples: None,
                },
                extensions: Default::default(),
            },
            Parameter {
                name: "hub.verify_token".to_owned(),
                location: "query".to_owned(),
                description: Some("Verification token".to_owned()),
                required,
                deprecated: false,
                allow_empty_value: false,
                value: ParameterValue::Schema {
                    style: None,
                    explode: None,
                    allow_reserved: false,
                    schema: gen.json_schema::<String>(),
                    example: Some(serde_json::json!("meatyhamhock")),
                    examples: None,
                },
                extensions: Default::default(),
            },
        ])
    }
}
