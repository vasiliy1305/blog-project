use crate::domain::error::DomainError;
use crate::domain::post::{CreatePost, Post, UpdatePost};
use sqlx::PgPool;

pub trait PostRepository {
    async fn create(&self, create_post: &CreatePost, author_id: i64) -> Result<Post, DomainError>;

    async fn find_by_id(&self, id: i64) -> Result<Option<Post>, DomainError>;

    async fn update(&self, id: i64, post: &UpdatePost) -> Result<Post, DomainError>;

    async fn delete(&self, id: i64) -> Result<(), DomainError>;

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError>;
}

pub struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresPostRepository { pool }
    }
}

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

    async fn update(&self, id: i64, post: &UpdatePost) -> Result<Post, DomainError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts
            SET
                title = $1,
                content = $2,
                updated_at = now()
            WHERE id = $3
            RETURNING
                id,
                author_id,
                title,
                content,
                created_at,
                updated_at
            "#,
        )
        .bind(&post.title)
        .bind(&post.content)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(post)
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError> {
        let posts = sqlx::query_as::<_, Post>(
            r#"
                    SELECT
                        id,
                        author_id,
                        title,
                        content,
                        created_at,
                        updated_at
                    FROM posts
                    ORDER BY created_at DESC
                    LIMIT $1
                    OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok(posts)
    }
}
