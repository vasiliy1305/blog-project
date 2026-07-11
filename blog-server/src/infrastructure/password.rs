use crate::domain::error::DomainError;
use argon2::{
    Algorithm, Argon2, Params, PasswordVerifier, Version,
    password_hash::{
        self, PasswordHash, PasswordHasher, SaltString,
        rand_core::OsRng,
    },
};

pub trait Password {
    fn hash_password(password: &str) -> Result<String, DomainError>;
    fn verify_password(password: &str, hash: &str) -> Result<bool, DomainError>;
}

pub struct PasswordArgon2 {}

impl Password for PasswordArgon2 {
    fn hash_password(password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(19 * 1024, 2, 1, None)
            .map_err(|error| DomainError::PasswordHash(error.to_string()))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|error| DomainError::PasswordHash(error.to_string()))?;
        Ok(password_hash.to_string())
    }

    fn verify_password(password: &str, hash: &str) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|error| DomainError::PasswordHash(error.to_string()))?;
        let argon2 = Argon2::default();
        let result = argon2.verify_password(password.as_bytes(), &parsed_hash);

        match result {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(error) => Err(DomainError::PasswordHash(error.to_string())),
        }
    }
}
