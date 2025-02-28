use thiserror::Error;
use tonic::codegen::http::uri::InvalidUri;

/// Qdrant client error
#[derive(Error, Debug)]
pub enum QdrantError {
    /// Qdrant server responded with an error
    #[error("Error in the response: {} {} {:?}", .status.code(), .status.message(), .status.metadata())]
    ResponseError {
        /// gRPC status code
        status: tonic::Status,
    },

    /// Conversion of a Rust into an API type failed
    ///
    /// Such error may include trying to convert a sparse vector into a dense vector.
    #[error("Error in conversion: {}", .0)]
    ConversionError(String),

    /// Invalid Qdrant server URI
    #[error("Invalid URI: {}", .0)]
    InvalidUri(#[source] InvalidUri),

    /// Snapshot not found
    #[error("No snapshot found for collection: {}", .0)]
    NoSnapshotFound(String),

    /// Generic IO error
    #[error("IO error: {}", .0)]
    Io(#[from] std::io::Error),

    /// API request error
    #[cfg(feature = "reqwest")]
    #[error("Reqwest error: {}", .0)]
    Reqwest(#[from] reqwest::Error),

    /// JSON to payload conversion error, only JSON objects are supported
    #[cfg(feature = "serde")]
    #[error("JSON cannot be converted to payload, only JSON objects are supported")]
    JsonToPayload(serde_json::Value),
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
