use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}