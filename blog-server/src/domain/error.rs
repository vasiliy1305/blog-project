use actix_web::cookie::time::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User {0} not found")]
    UserNotFound(u64),

    #[error("User {0} already exists")]
    UserAlreadyExists(String),

    #[error("Post {0} not found")]
    PostNotFound(u64),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Access forbidden")]
    Forbidden,

    #[error("Password hash error: {0}")]
    PasswordHash(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error), // немного портит архитектуру, но сильно упращает код
}
