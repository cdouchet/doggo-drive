use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use reqwest::StatusCode;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug)]
pub struct DoggoError {
    name: String,
    description: Option<String>,
    status_code: StatusCode,
}

#[derive(Serialize)]
struct SerializableDoggoError {
    name: String,
    description: Option<String>,
}

impl Display for DoggoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

impl ResponseError for DoggoError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponseBuilder::new(self.status_code).body(self.to_json())
    }

    fn status_code(&self) -> reqwest::StatusCode {
        self.status_code
    }
}

impl DoggoError {
    fn to_json(&self) -> String {
        let serializable = SerializableDoggoError {
            name: self.name.clone(),
            description: self.description.clone(),
        };
        serde_json::to_string(&serializable).unwrap()
    }
}
