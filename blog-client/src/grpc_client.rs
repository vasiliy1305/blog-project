use blog_proto::blog::{
    CreatePostRequest, DeletePostRequest, GetPostRequest, ListPostsRequest, LoginRequest,
    Post as ProtoPost, RegisterRequest, UpdatePostRequest, User as ProtoUser,
    blog_service_client::BlogServiceClient,
};
use tonic::{Request, metadata::MetadataValue};

use chrono::{DateTime, Utc};
use tonic::transport::Channel;

use crate::error::BlogClientError;
use crate::models::{AuthResponse, Post, UserResponse};

#[derive(Debug, Clone)]
pub struct GrpcClient {
    client: BlogServiceClient<Channel>,
}

impl GrpcClient {
    pub async fn connect(server_url: String) -> Result<Self, BlogClientError> {
        let server_url = normalize_grpc_url(server_url);

        let client = BlogServiceClient::connect(server_url).await?;

        Ok(Self { client })
    }

    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError> {
        let request = RegisterRequest {
            username,
            email,
            password,
        };

        let response = self.client.register(request).await?.into_inner();

        let user = response.user.ok_or_else(|| {
            BlogClientError::InvalidRequest(
                "gRPC register response does not contain user".to_owned(),
            )
        })?;

        Ok(AuthResponse {
            user: convert_user(user)?,
            token: response.access_token,
        })
    }

    pub async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError> {
        let request = LoginRequest { username, password };

        let response = self.client.login(request).await?.into_inner();

        let user = response.user.ok_or_else(|| {
            BlogClientError::InvalidRequest("gRPC login response does not contain user".to_owned())
        })?;

        Ok(AuthResponse {
            user: convert_user(user)?,
            token: response.access_token,
        })
    }

    pub async fn get_post(&mut self, id: i64) -> Result<Post, BlogClientError> {
        let request = GetPostRequest { id };

        let response = self.client.get_post(request).await?.into_inner();

        let post = response.post.ok_or_else(|| {
            BlogClientError::InvalidRequest(
                "gRPC get_post response does not contain post".to_owned(),
            )
        })?;

        convert_post(post)
    }

    pub async fn list_posts(
        &mut self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, BlogClientError> {
        let request = ListPostsRequest { limit, offset };

        let response = self.client.list_posts(request).await?.into_inner();

        response.posts.into_iter().map(convert_post).collect()
    }

    pub async fn create_post(
        &mut self,
        token: &str,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError> {
        let message = CreatePostRequest { title, content };
        let request = authenticated_request(message, token)?;

        let response = self.client.create_post(request).await?.into_inner();

        let post = response.post.ok_or_else(|| {
            BlogClientError::InvalidRequest(
                "gRPC create_post response does not contain post".to_owned(),
            )
        })?;

        convert_post(post)
    }

    pub async fn update_post(
        &mut self,
        token: &str,
        id: i64,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError> {
        let message = UpdatePostRequest { id, title, content };

        let request = authenticated_request(message, token)?;

        let response = self.client.update_post(request).await?.into_inner();

        let post = response.post.ok_or_else(|| {
            BlogClientError::InvalidRequest(
                "gRPC update_post response does not contain post".to_owned(),
            )
        })?;

        convert_post(post)
    }

    pub async fn delete_post(&mut self, token: &str, id: i64) -> Result<(), BlogClientError> {
        let message = DeletePostRequest { id };
        let request = authenticated_request(message, token)?;

        let response = self.client.delete_post(request).await?.into_inner();

        if !response.success {
            return Err(BlogClientError::InvalidRequest(
                "gRPC server did not confirm post deletion".to_owned(),
            ));
        }

        Ok(())
    }
}

fn convert_user(user: ProtoUser) -> Result<UserResponse, BlogClientError> {
    let created_at = DateTime::parse_from_rfc3339(&user.created_at)
        .map_err(|error| BlogClientError::InvalidRequest(error.to_string()))?
        .with_timezone(&Utc);

    Ok(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at,
    })
}

fn normalize_grpc_url(server_url: String) -> String {
    let server_url = server_url.trim_end_matches('/');

    if server_url.starts_with("http://") || server_url.starts_with("https://") {
        server_url.to_owned()
    } else {
        format!("http://{server_url}")
    }
}

fn convert_post(post: ProtoPost) -> Result<Post, BlogClientError> {
    let created_at = DateTime::parse_from_rfc3339(&post.created_at)
        .map_err(|error| BlogClientError::InvalidRequest(error.to_string()))?
        .with_timezone(&Utc);

    let updated_at = DateTime::parse_from_rfc3339(&post.updated_at)
        .map_err(|error| BlogClientError::InvalidRequest(error.to_string()))?
        .with_timezone(&Utc);

    Ok(Post {
        id: post.id,
        author_id: post.author_id,
        title: post.title,
        content: post.content,
        created_at,
        updated_at,
    })
}

fn authenticated_request<T>(message: T, token: &str) -> Result<Request<T>, BlogClientError> {
    let mut request = Request::new(message);

    let authorization = format!("Bearer {token}").parse::<MetadataValue<_>>()?;

    request
        .metadata_mut()
        .insert("authorization", authorization);

    Ok(request)
}
