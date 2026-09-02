//! Client for Qdrant Serverless.
//!
//! **In development — do not use yet.** This API is experimental and unstable;
//! it may change without notice and is not ready for production or general use.
//!
//! Point-level operations (query, upsert, ...) behave like the regular client;
//! collection management uses the simplified, tenant-facing serverless API.
//!
//! # Example
//!
//! ```no_run
//! use qdrant_client::serverless::{
//!     CollectionConfig, DenseVectorConfig, Distance, QdrantServerless,
//! };
//!
//! # async fn run() -> Result<(), qdrant_client::QdrantError> {
//! let client = QdrantServerless::from_url("https://serverless.example.cloud.qdrant.io")
//!     .api_key("<your api key>")
//!     .build()?;
//!
//! client
//!     .create_collection(
//!         "my-collection",
//!         CollectionConfig::new()
//!             .dense_vector(DenseVectorConfig::new(1536, Distance::Cosine)),
//!     )
//!     .await?;
//! # Ok(())
//! # }
//! ```

mod client;
mod conversions;
#[allow(clippy::all)]
#[rustfmt::skip]
pub(crate) mod grpc;
pub mod models;

pub use client::{QdrantServerless, QdrantServerlessBuilder, DEFAULT_SERVERLESS_GRPC_PORT};
pub use models::{
    BoolIndex, CollectionConfig, CollectionInfo, CollectionSummary, DatetimeIndex,
    DenseVectorConfig, Distance, FloatIndex, GeoIndex, IntegerIndex, KeywordIndex, PayloadIndex,
    PrecisionTier, SparseVectorConfig, TextIndex, Tokenizer, UuidIndex,
};
