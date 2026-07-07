use crate::domain::error::DomainError;
use crate::domain::post::{CreatePost, Post, UpdatePost};
use sqlx::PgPool;
// use tonic::Code::Ok;

pub trait PostRepository {
    async fn create(&self, create_post: &CreatePost, author_id: i64) -> Result<Post, DomainError>;

    async fn find_by_id(&self, id: i64) -> Result<Option<Post>, DomainError>;

    async fn update(&self, id: i64, post: &UpdatePost) -> Result<Post, DomainError>;

    async fn delete(&self, id: i64) -> Result<(), DomainError>;

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Post>, DomainError>;
}

pub struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresPostRepository { pool }
    }
}

// CREATE TABLE IF NOT EXISTS posts(
//     id BIGSERIAL PRIMARY KEY,
//     title VARCHAR NOT NULL,
//     content TEXT,
//     author_id BIGINT NOT NULL,
//     created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT  now(),
//     updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT  now(),

//     FOREIGN KEY (author_id)
//     REFERENCES  users(id)
//     ON DELETE CASCADE
// );

impl PostRepository for PostgresPostRepository {
    async fn create(&self, create_post: &CreatePost, author_id: i64) -> Result<Post, DomainError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (title, content, author_id)
            VALUES ($1, $2, $3)
            RETURNING
            id,
            title,
            content,
            author_id,
            created_at,
            updated_at"#,
        )
        .bind(&create_post.title)
        .bind(&create_post.content)
        .bind(author_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(post)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Post>, DomainError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            SELECT 
                id,
                title,
                content,
                author_id,
                created_at,
                updated_at
            FROM posts 
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(post)
    }

    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        sqlx::query(
            r#"
                DELETE FROM posts
                WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Post>, DomainError> {}
    async fn update(&self, id: i64, post: &UpdatePost) -> Result<Post, DomainError> {}
}
