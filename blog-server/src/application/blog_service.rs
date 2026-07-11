use crate::data::post_repository::PostRepository;
use crate::domain::error::DomainError;
use crate::domain::post::{CreatePost, Post, UpdatePost};

pub struct BlogService<R>
where
    R: PostRepository,
{
    repository: R,
}

impl<R> BlogService<R>
where
    R: PostRepository,
{
    pub fn new(repository: R) -> Self {
        BlogService { repository }
    }

    pub async fn create(
        &self,
        create_post: &CreatePost,
        author_id: i64,
    ) -> Result<Post, DomainError> {
        if create_post.title.trim().is_empty() {
            return Err(DomainError::Validation(
                "title must not be empty".to_string(),
            ));
        }
        self.repository.create(create_post, author_id).await
    }

    pub async fn get(&self, id: i64) -> Result<Post, DomainError> {
        match self.repository.find_by_id(id).await? {
            Some(post) => Ok(post),
            None => Err(DomainError::PostNotFound(id)),
        }
    }

    pub async fn update(
        &self,
        post_id: i64,
        author_id: i64,
        post: &UpdatePost,
    ) -> Result<Post, DomainError> {
        self.verify_post_author(post_id, author_id).await?;
        if post.title.trim().is_empty() {
            return Err(DomainError::Validation(
                "title must not be empty".to_string(),
            ));
        }
        self.repository.update(post_id, post).await
    }

    pub async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError> {
        if offset < 0 {
            return Err(DomainError::Validation(
                "offset must not be negative".to_string(),
            ));
        }

        if limit <= 0 {
            return Err(DomainError::Validation(
                "limit must be greater than zero".to_string(),
            ));
        }
        self.repository.list(limit, offset).await
    }

    pub async fn delete(&self, post_id: i64, author_id: i64) -> Result<(), DomainError> {
        self.verify_post_author(post_id, author_id).await?;
        self.repository.delete(post_id).await
    }

    async fn verify_post_author(&self, post_id: i64, author_id: i64) -> Result<(), DomainError> {
        let post = self.get(post_id).await?;
        if post.author_id != author_id {
            return Err(DomainError::Forbidden);
        }
        Ok(())
    }
}
