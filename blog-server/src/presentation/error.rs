use actix_web::{ResponseError, http::StatusCode};
use crate::domain::error::DomainError::{self, PostNotFound};

impl ResponseError for DomainError{
    fn status_code(&self) -> StatusCode {
        match self {
            DomainError::UserNotFound(_)
            | DomainError::PostNotFound(_) => StatusCode::NOT_FOUND,
            DomainError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            DomainError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            DomainError::Forbidden => StatusCode::FORBIDDEN,
            DomainError::Validation(_) => StatusCode::BAD_REQUEST,
            DomainError::Registration(_) 
            | DomainError::PasswordHash(_)
            | DomainError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}


// UserNotFound        → 404 Not Found
// PostNotFound        → 404 Not Found
// UserAlreadyExists   → 409 Conflict
// InvalidCredentials  → 401 Unauthorized
// Forbidden           → 403 Forbidden
// Validation          → 400 Bad Request
// Registration        → 400 или 500 — зависит от смысла
// PasswordHash        → 500 Internal Server Error
// Database            → 500 Internal Server Error