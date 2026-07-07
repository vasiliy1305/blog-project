use crate::domain::error::DomainError;
use crate::domain::post::{CreatePost, Post, UpdatePost};

pub trait PostRepository {
    async fn create(&self, create_post: &CreatePost) -> Result<Post, DomainError>;

    async fn find_by_id(&self, id: i64) -> Result<Option<Post>, DomainError>;

    async fn update(&self, id: i64, post: &UpdatePost) -> Result<Post, DomainError>;

    async fn delete(&self, id: i64) -> Result<(), DomainError>;

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Post>, DomainError>;
}
