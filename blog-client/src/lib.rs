pub mod error;
pub mod grpc_client;
pub mod http_client;
pub mod models;

use error::BlogClientError;
use grpc_client::GrpcClient;
use http_client::HttpClient;
use models::{AuthResponse, Post};

#[derive(Debug)]
pub enum Transport {
    Http(HttpClient),
    Grpc(GrpcClient),
}

#[derive(Debug)]
pub struct BlogClient {
    transport: Transport,
    token: Option<String>,
}

impl BlogClient {
    pub fn new_http(base_url: String) -> Result<Self, BlogClientError> {
        let http_client = HttpClient::new(base_url)?;

        Ok(Self {
            transport: Transport::Http(http_client),
            token: None,
        })
    }

    pub async fn new_grpc(server_url: String) -> Result<Self, BlogClientError> {
        let grpc_client = GrpcClient::connect(server_url).await?;

        Ok(Self {
            transport: Transport::Grpc(grpc_client),
            token: None,
        })
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError> {
        let response = match &mut self.transport {
            Transport::Http(client) => client.register(username, email, password).await?,
            Transport::Grpc(client) => client.register(username, email, password).await?,
        };

        self.token = Some(response.token.clone());
        Ok(response)
    }

    pub async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError> {
        let response = match &mut self.transport {
            Transport::Http(client) => client.login(username, password).await?,
            Transport::Grpc(client) => client.login(username, password).await?,
        };

        self.token = Some(response.token.clone());
        Ok(response)
    }

    pub async fn get_post(&mut self, id: i64) -> Result<Post, BlogClientError> {
        match &mut self.transport {
            Transport::Http(client) => client.get_post(id).await,
            Transport::Grpc(client) => client.get_post(id).await,
        }
    }

    pub async fn list_posts(
        &mut self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, BlogClientError> {
        match &mut self.transport {
            Transport::Http(client) => client.list_posts(limit, offset).await,
            Transport::Grpc(client) => client.list_posts(limit, offset).await,
        }
    }

    pub async fn create_post(
        &mut self,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError> {
        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;

        match &mut self.transport {
            Transport::Http(client) => client.create_post(token, title, content).await,
            Transport::Grpc(client) => client.create_post(token, title, content).await,
        }
    }

    pub async fn update_post(
        &mut self,
        id: i64,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError> {
        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;

        match &mut self.transport {
            Transport::Http(client) => client.update_post(token, id, title, content).await,
            Transport::Grpc(client) => client.update_post(token, id, title, content).await,
        }
    }

    pub async fn delete_post(&mut self, id: i64) -> Result<(), BlogClientError> {
        let token = self.token.as_deref().ok_or(BlogClientError::Unauthorized)?;

        match &mut self.transport {
            Transport::Http(client) => client.delete_post(token, id).await,
            Transport::Grpc(client) => client.delete_post(token, id).await,
        }
    }
}
