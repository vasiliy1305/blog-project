use std::sync::Arc;
use tonic::{Request, Response, Status, metadata::MetadataMap};

use blog_proto::blog::Post as ProtoPost;
use blog_proto::blog::{
    CreatePostRequest, CreatePostResponse, DeletePostRequest, DeletePostResponse, GetPostRequest,
    GetPostResponse, ListPostsRequest, ListPostsResponse, LoginRequest, LoginResponse,
    RegisterRequest, RegisterResponse, UpdatePostRequest, UpdatePostResponse,
};

use crate::application::{auth_service::AuthService, blog_service::BlogService};
use crate::data::{
    post_repository::PostgresPostRepository, user_repository::PostgresUserRepository,
};
use crate::domain::error::DomainError;
use crate::domain::post::Post as DomainPost;
use crate::domain::post::{CreatePost, UpdatePost};
use crate::domain::user::{Login, Registration};

use crate::infrastructure::jwt::{Claims, JwtService};
use crate::infrastructure::password::PasswordArgon2;

pub struct BlogGrpcService {
    auth_service: Arc<AuthService<PostgresUserRepository, PasswordArgon2>>,
    blog_service: Arc<BlogService<PostgresPostRepository>>,
    jwt_service: Arc<JwtService>,
}

impl BlogGrpcService {
    pub fn new(
        auth_service: Arc<AuthService<PostgresUserRepository, PasswordArgon2>>,
        blog_service: Arc<BlogService<PostgresPostRepository>>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            auth_service,
            blog_service,
            jwt_service,
        }
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostResponse>, Status> {
        let request = request.into_inner();

        let post = self
            .blog_service
            .get(request.id)
            .await
            .map_err(domain_error_to_status)?;

        let response = GetPostResponse {
            post: Some(post_to_proto(post)),
        };
        Ok(Response::new(response))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let request = request.into_inner();

        let posts = self
            .blog_service
            .list(request.limit, request.offset)
            .await
            .map_err(domain_error_to_status)?;

        let proto_posts: Vec<ProtoPost> = posts.into_iter().map(post_to_proto).collect();

        let responce = ListPostsResponse { posts: proto_posts };
        Ok(Response::new(responce))
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let request = request.into_inner();

        let user = self
            .auth_service
            .register(&Registration {
                username: request.username,
                email: request.email,
                password: request.password,
            })
            .await
            .map_err(domain_error_to_status)?;

        let responce = RegisterResponse {
            user_id: user.user.id,
            access_token: user.token,
        };
        Ok(Response::new(responce))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();

        let user = self
            .auth_service
            .login(Login {
                username: request.username,
                password: request.password,
            })
            .await
            .map_err(domain_error_to_status)?;

        let responce = LoginResponse {
            access_token: user.token,
        };
        Ok(Response::new(responce))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostResponse>, Status> {
        let claims = self.get_claims(request.metadata())?;
        let request = request.into_inner();

        let post = self
            .blog_service
            .create(
                &CreatePost {
                    title: request.title,
                    content: request.content,
                },
                claims.user_id,
            )
            .await
            .map_err(domain_error_to_status)?;

        let responce = CreatePostResponse {
            post: Some(post_to_proto(post)),
        };

        Ok(Response::new(responce))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<UpdatePostResponse>, Status> {
        let claims = self.get_claims(request.metadata())?;
        let request = request.into_inner();

        let post = self
            .blog_service
            .update(
                request.id,
                claims.user_id,
                &UpdatePost {
                    title: request.title,
                    content: request.content,
                },
            )
            .await
            .map_err(domain_error_to_status)?;

        let responce = UpdatePostResponse {
            post: Some(post_to_proto(post)),
        };

        Ok(Response::new(responce))
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let claims = self.get_claims(request.metadata())?;
        let request = request.into_inner();

        let post = self
            .blog_service
            .delete(request.id, claims.user_id)
            .await
            .map_err(domain_error_to_status)?;

        let responce = DeletePostResponse { success: true };

        Ok(Response::new(responce))
    }

    fn get_claims(&self, metadata: &MetadataMap) -> Result<Claims, Status> {
        let authorization = metadata
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("missing authorization metadata"))?;

        let authorization = authorization
            .to_str()
            .map_err(|_| Status::unauthenticated("invalid authorization metadata"))?;

        let token = authorization
            .strip_prefix("Bearer ")
            .ok_or_else(|| Status::unauthenticated("expected Bearer token"))?;

        self.jwt_service
            .verify_token(token)
            .map_err(|_| Status::unauthenticated("invalid or expired token"))
    }
}

fn domain_error_to_status(error: DomainError) -> Status {
    match error {
        DomainError::UserNotFound(message) => Status::not_found(message),

        DomainError::UserAlreadyExists(message) => Status::already_exists(message),

        DomainError::PostNotFound(id) => Status::not_found(format!("post {id} not found")),

        DomainError::InvalidCredentials => Status::unauthenticated("invalid credentials"),

        DomainError::Forbidden => Status::permission_denied("access forbidden"),

        DomainError::Validation(message) => Status::invalid_argument(message),

        DomainError::PasswordHash(message) | DomainError::Registration(message) => {
            Status::internal(message)
        }

        DomainError::Database(error) => Status::internal(error.to_string()),
    }
}

fn post_to_proto(post: DomainPost) -> ProtoPost {
    ProtoPost {
        id: post.id,
        author_id: post.author_id,
        title: post.title,
        content: post.content,
        created_at: post.created_at.to_rfc3339(),
        updated_at: post.updated_at.to_rfc3339(),
    }
}
