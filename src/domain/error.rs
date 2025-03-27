#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Entity not found: {0}")]
    NotFoundError(String),
    
    #[error("Unauthorized action: {0}")]
    UnauthorizedError(String),
    
    #[error("Resource conflict: {0}")]
    ConflictError(String),
    
    #[error("Domain logic error: {0}")]
    LogicError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
} 