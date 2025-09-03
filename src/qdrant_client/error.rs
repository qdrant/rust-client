use thiserror::Error;
use tonic::codegen::http::uri::InvalidUri;

#[cfg(feature = "serde")]
use crate::serde_deser::DeserPayloadError;

/// Qdrant client error
#[derive(Error, Debug)]
pub enum QdrantError {
    /// Qdrant server responded with an error
    #[error("Error in the response: {} {} {:?}", .status.code(), .status.message(), .status.metadata())]
    ResponseError {
        /// gRPC status code
        status: tonic::Status,
    },

    /// Qdrant server responded with a resource exhausted error
    #[error("Resource exhausted: {} {} {:?}, retry after {} seconds", .status.code(), .status.message(), .status.metadata(), .retry_after_seconds)]
    ResourceExhaustedError {
        /// gRPC status code
        status: tonic::Status,
        /// Retry after seconds
        retry_after_seconds: u64,
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

    /// Error when failing to deserializing payload using `payload.deserialize()`.
    #[cfg(feature = "serde")]
    #[error("Error in payload deserialization")]
    PayloadDeserialization(#[from] DeserPayloadError),
}

impl QdrantError {
    // Only used in tests for now.
    #[cfg(feature = "serde")]
    #[allow(dead_code)]
    pub(crate) fn as_payload_deserialization(&self) -> Option<&DeserPayloadError> {
        if let QdrantError::PayloadDeserialization(err) = self {
            Some(err)
        } else {
            None
        }
    }
}

impl From<tonic::Status> for QdrantError {
    fn from(status: tonic::Status) -> Self {
        if status.code() == tonic::Code::ResourceExhausted {
            if let Some(retry_after_value) = status
                .metadata()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
            {
                return QdrantError::ResourceExhaustedError {
                    status,
                    retry_after_seconds: retry_after_value,
                };
            }
        }

        QdrantError::ResponseError { status }
    }
}

impl From<InvalidUri> for QdrantError {
    fn from(err: InvalidUri) -> Self {
        QdrantError::InvalidUri(err)
    }
}
