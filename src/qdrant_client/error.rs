use thiserror::Error;
use tonic::codegen::http::uri::InvalidUri;

/// Qdrant client error
#[derive(Error, Debug)]
pub enum Error {
    /// API response error
    #[error("Error in the response: {}", .status.code())]
    ResponseError {
        /// gRPC status code
        status: tonic::Status,
    },

    /// Invalid Qdrant server URI
    #[error("Invalid URI: {}", .0)]
    InvalidUri(#[source] InvalidUri),

    /// Snapshot not found
    #[error("No snapshot found for collection: {}", .0)]
    NoSnapshotFound(String),

    /// IO error
    #[error("IO error: {}", .0)]
    Io(#[from] std::io::Error),

    /// API request error
    #[cfg(feature = "reqwest")]
    #[error("Reqwest error: {}", .0)]
    Reqwest(#[from] reqwest::Error),
}

impl From<tonic::Status> for Error {
    fn from(status: tonic::Status) -> Self {
        Error::ResponseError { status }
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Self {
        Error::InvalidUri(err)
    }
}
