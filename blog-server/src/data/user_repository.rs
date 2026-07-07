use crate::domain::error::DomainError;
use crate::domain::user::{CreateUser, User};

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

pub trait UserRepository {
    async fn create(&self, create_user_info: &CreateUser) -> Result<User, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
}

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresUserRepository { pool: pool }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn create(&self, create_user_info: CreateUser) -> Result<User, DomainError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(create_user_info.username)
        .bind(create_user_info.email)
        .bind(create_user_info.password_hash)
        .execute(self.pool)
        .await?;
    
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {}
}
