use crate::data::post_repository::PostRepository;
use crate::domain::error::DomainError;
use crate::domain::post::{Post, CreatePost, UpdatePost};
use crate::infrastructure::jwt::JwtService;
use crate::infrastructure::password::Password;


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
    pub fn new(jwt: JwtService, repository: R) -> Self {
        BlogService {
            repository,
        }
    }

}


