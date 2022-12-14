use utoipa::ToSchema;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::route::dto;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResponseError {
    pub code: u16,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl ResponseError {
    pub fn new(status: StatusCode, message: Option<String>) -> Self {
        Self {
            code: status.as_u16(),
            error: status.canonical_reason().map(|v| v.to_string()),
            message,
        }
    }

    pub fn from_error(error: impl std::error::Error + actix_web::ResponseError) -> ResponseError {
        ResponseError::new(error.status_code(), Some(error.to_string()))
    }

    pub fn internal_error_example() -> dto::error::ResponseError {
        ResponseError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("Something went wrong".to_string()),
        )
    }
}