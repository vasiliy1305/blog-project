use crate::data::user_repository::UserRepository;
use crate::domain::error::DomainError;
use crate::domain::user::{CreateUser, Login, Registration, User};
use crate::infrastructure::jwt::JwtService;
use crate::infrastructure::password::Password;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::sync::Arc;

pub struct AuthService<R, P>
where
    R: UserRepository,
    P: Password,
{
    jwt: Arc<JwtService>,
    repository: R,
    password: P,
}

impl<R, P> AuthService<R, P>
where
    R: UserRepository,
    P: Password,
{
    pub fn new(jwt: Arc<JwtService>, repository: R, password: P) -> Self {
        AuthService {
            jwt,
            repository,
            password,
        }
    }

    pub async fn register(&self, reg_info: &Registration) -> Result<AuthResponse, DomainError> {
        match self.repository.find_by_username(&reg_info.username).await? {
            Some(user) => Err(DomainError::UserAlreadyExists(user.username)),
            None => {
                let hash = self.password.hash_password(&reg_info.password)?;
                let user = self
                    .repository
                    .create(&CreateUser {
                        username: reg_info.username.clone(),
                        email: reg_info.email.clone(),
                        password_hash: hash,
                    })
                    .await?;

                match self.jwt.generate_token(user.id, &user.username) {
                    Err(error) => Err(DomainError::Registration(error.to_string())),
                    Ok(token) => Ok(AuthResponse {
                        user: user.into(),
                        token: token,
                    }),
                }
            }
        }
    }

    pub async fn login(&self, login_info: Login) -> Result<AuthResponse, DomainError> {
        match self
            .repository
            .find_by_username(&login_info.username)
            .await?
        {
            Some(user) => {
                if self
                    .password
                    .verify_password(&login_info.password, &user.password_hash)?
                {
                    match self.jwt.generate_token(user.id, &user.username) {
                        Err(error) => Err(DomainError::Validation(error.to_string())),
                        Ok(token) => Ok(AuthResponse {
                            user: user.into(),
                            token: token,
                        }),
                    }
                } else {
                    Err(DomainError::InvalidCredentials)
                }
            }
            None => Err(DomainError::InvalidCredentials),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
