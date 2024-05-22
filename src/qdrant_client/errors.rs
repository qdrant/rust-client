use thiserror::Error;
use tonic::codegen::http::uri::InvalidUri;

#[derive(Error, Debug)]
pub enum QdrantError {
    #[error("Error in the response: {}", .status.code())]
    ResponseError { status: tonic::Status },

    #[error("Invalid URI: {}", .0)]
    InvalidUri(#[source] InvalidUri),

    #[error("No snapshot found for collection: {}", .0)]
    NoSnapshotFound(String),

    #[error("IO error: {}", .0)]
    Io(#[from] std::io::Error),

    #[cfg(feature = "reqwest")]
    #[error("Reqwest error: {}", .0)]
    Reqwest(#[from] reqwest::Error),

    /// An error for failed conversions (e.g. calling `String::try_from(v)`
    /// on an integer [`Value`](Value))
    #[error("Type mismatch. Expected \"{}\"", .0)]
    TypeMismatch(String),
}

impl From<tonic::Status> for QdrantError {
    fn from(status: tonic::Status) -> Self {
        QdrantError::ResponseError { status }
    }
}

impl From<InvalidUri> for QdrantError {
    fn from(err: InvalidUri) -> Self {
        QdrantError::InvalidUri(err)
    }
}
