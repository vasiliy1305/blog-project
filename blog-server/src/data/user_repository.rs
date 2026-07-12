use crate::domain::error::DomainError;
use crate::domain::user::{CreateUser, User};
use sqlx::PgPool;

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
    async fn create(&self, create_user_info: &CreateUser) -> Result<User, DomainError> {
        let user = sqlx::query_as::<_, User>(
            r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING
            id,
            username,
            email,
            password_hash,
            created_at
        "#,
        )
        .bind(&create_user_info.username)
        .bind(&create_user_info.email)
        .bind(&create_user_info.password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let user = sqlx::query_as::<_, User>(
            r#"
    SELECT id, username, email, password_hash, created_at
    FROM users
    WHERE username = $1
    "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
}
