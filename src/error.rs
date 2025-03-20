use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

/// 應用程式錯誤類型
/// 
/// # 變體
/// 
/// - `ConfigError`: 配置相關錯誤
/// - `ExternalServiceError`: 外部服務錯誤
/// - `InternalError`: 內部服務器錯誤
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

/// 為 AppError 實現 ResponseError trait
/// 
/// # 功能
/// 
/// 將應用程式錯誤轉換為適當的 HTTP 響應
/// 
/// # 響應
/// 
/// - `ConfigError`: 500 Internal Server Error
/// - `ExternalServiceError`: 503 Service Unavailable
/// - `InternalError`: 500 Internal Server Error
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