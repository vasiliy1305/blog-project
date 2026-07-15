use blog_proto::blog::blog_service_client::BlogServiceClient;
use tonic::transport::Channel;

use crate::error::BlogClientError;

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
}

fn normalize_grpc_url(server_url: String) -> String {
    let server_url = server_url.trim_end_matches('/');

    if server_url.starts_with("http://") || server_url.starts_with("https://") {
        server_url.to_owned()
    } else {
        format!("http://{server_url}")
    }
}
