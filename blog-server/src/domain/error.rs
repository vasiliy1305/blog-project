use actix_web::cookie::time::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User {0} not found")]
    UserNotFound(String),

    #[error("User {0} already exists")]
    UserAlreadyExists(String),

    #[error("Post {0} not found")]
    PostNotFound(i64),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Access forbidden")]
    Forbidden,

    #[error("Password hash error: {0}")]
    PasswordHash(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Registration failed: {0}")]
    Registration(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error), // немного портит архитектуру, но сильно упращает код
}
