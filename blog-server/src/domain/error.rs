use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Password hash error: {0}")]
    PasswordHash(String),
}
