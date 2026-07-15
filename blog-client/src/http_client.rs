use std::time::Duration;

use reqwest::Client;

use crate::error::BlogClientError;
use crate::models::{
    AuthResponse, CreatePostRequest, LoginRequest, Post, RegisterRequest, UpdatePostRequest,
};

#[derive(Debug, Clone)]
pub struct HttpClient {
    base_url: String,
    client: Client,
}

impl HttpClient {
    pub fn new(base_url: String) -> Result<Self, BlogClientError> {
        let base_url = base_url.trim_end_matches('/').to_owned();

        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

        Ok(Self { base_url, client })
    }

    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError> {
        let url = format!("{}/api/auth/register", self.base_url);

        let request = RegisterRequest {
            username,
            email,
            password,
        };

        let response = self.client.post(url).json(&request).send().await?;
        let response = check_response(response).await?;
        let auth_response = response.json::<AuthResponse>().await?;

        Ok(auth_response)
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError> {
        let url = format!("{}/api/auth/login", self.base_url);
        let request = LoginRequest { username, password };
        let response = self.client.post(url).json(&request).send().await?;
        let response = check_response(response).await?;
        let auth_response = response.json::<AuthResponse>().await?;

        Ok(auth_response)
    }

    pub async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        let url = format!("{}/api/posts/{id}", self.base_url);

        let response = self.client.get(url).send().await?;
        let response = check_response(response).await?;
        let post = response.json::<Post>().await?;

        Ok(post)
    }

    pub async fn list_posts(&self, limit: i64, offset: i64) -> Result<Vec<Post>, BlogClientError> {
        let url = format!("{}/api/posts", self.base_url);

        let response = self
            .client
            .get(url)
            .query(&[("limit", limit), ("offset", offset)])
            .send()
            .await?;

        let response = check_response(response).await?;

        let posts = response.json::<Vec<Post>>().await?;

        Ok(posts)
    }

    pub async fn create_post(
        &self,
        token: &str,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError> {
        let url = format!("{}/api/posts", self.base_url);

        let request = CreatePostRequest { title, content };

        let response = self
            .client
            .post(url)
            .bearer_auth(token)
            .json(&request)
            .send()
            .await?;

        let response = check_response(response).await?;
        let post = response.json::<Post>().await?;

        Ok(post)
    }

    pub async fn update_post(
        &self,
        token: &str,
        id: i64,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError> {
        let url = format!("{}/api/posts/{id}", self.base_url);
        let request = UpdatePostRequest { title, content };

        let response = self
            .client
            .put(url)
            .bearer_auth(token)
            .json(&request)
            .send()
            .await?;

        let response = check_response(response).await?;
        let post = response.json::<Post>().await?;
        Ok(post)
    }

    pub async fn delete_post(&self, token: &str, id: i64) -> Result<(), BlogClientError> {
        let url = format!("{}/api/posts/{id}", self.base_url);
        let response = self.client.delete(url).bearer_auth(token).send().await?;
        check_response(response).await?;
        Ok(())
    }
}

async fn check_response(response: reqwest::Response) -> Result<reqwest::Response, BlogClientError> {
    let status = response.status();

    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(BlogClientError::Unauthorized);
    }

    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(BlogClientError::NotFound);
    }

    if !status.is_success() {
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "unknown server error".to_owned());

        return Err(BlogClientError::HttpStatus { status, message });
    }

    Ok(response)
}
