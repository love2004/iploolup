use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ConfigError(msg) => HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": msg})),
            AppError::ExternalServiceError(msg) => HttpResponse::ServiceUnavailable()
                .json(json!({"status": "error", "message": msg})),
            AppError::InternalError(msg) => HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": msg})),
        }
    }
}