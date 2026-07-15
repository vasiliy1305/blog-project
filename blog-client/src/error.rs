use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlogClientError {
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("server returned status {status}: {message}")]
    HttpStatus {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("gRPC request error: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("gRPC transport error: {0}")]
    GrpcTransport(#[from] tonic::transport::Error),

    #[error("invalid gRPC metadata: {0}")]
    InvalidMetadata(#[from] tonic::metadata::errors::InvalidMetadataValue),

    #[error("authentication token is missing")]
    Unauthorized,

    #[error("requested resource was not found")]
    NotFound,

    #[error("invalid request: {0}")]
    InvalidRequest(String),
}
