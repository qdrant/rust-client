//! Client for Qdrant Serverless.
//!
//! **In development — do not use yet.** This API is experimental and unstable;
//! it may change without notice and is not ready for production or general use.
//!
//! Serverless exposes the same point-level API as a regular Qdrant cluster (minus
//! read consistency, shard selection, write ordering and filtered updates), but a
//! much simpler, tenant-facing collection management API. Point operations are
//! delegated to the regular gRPC client; collection operations talk to the
//! serverless CollectionsService.

use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Uri};
use tonic::Status;

use crate::auth::MetadataInterceptor;
use crate::config::{AsOptionApiKey, AsTimeout, CompressionEncoding, QdrantConfig};
use crate::qdrant::{
    ClearPayloadPoints, CountPoints, CountResponse, DeletePayloadPoints, DeletePointVectors,
    DeletePoints, GetPoints, GetResponse, PointsOperationResponse, QueryBatchPoints,
    QueryBatchResponse, QueryGroupsResponse, QueryPointGroups, QueryPoints, QueryResponse,
    ScrollPoints, ScrollResponse, SetPayloadPoints, UpdateBatchPoints, UpdateBatchResponse,
    UpdatePointVectors, UpsertPoints,
};
use crate::qdrant_client::QdrantResult;
use crate::serverless::conversions::{collection_config_from_grpc, collection_config_to_grpc};
use crate::serverless::grpc::collections_service_client::CollectionsServiceClient;
use crate::serverless::grpc::{
    CreateCollectionRequest, DeleteCollectionRequest, GetCollectionRequest, ListCollectionsRequest,
};
use crate::serverless::models::{CollectionConfig, CollectionInfo, CollectionSummary};
use crate::Qdrant;

/// Default gRPC port for serverless when the URL omits an explicit port.
///
/// Serverless is exposed on the standard TLS port, not on Qdrant's 6334.
pub const DEFAULT_SERVERLESS_GRPC_PORT: u16 = 443;

/// Entry point to a Qdrant Serverless space.
///
/// **In development — do not use yet.** This client is experimental and unstable;
/// the API may change without notice and is not ready for production or general use.
///
/// Point operations behave like in the regular [`Qdrant`] client, except that
/// parameters serverless does not support (read consistency, shard selection,
/// write ordering, filtered updates, cross-collection lookups) are cleared
/// before the request is sent. Collection management uses the simplified
/// serverless API: only the tenant-facing configuration is exposed; storage
/// internals (quantization, WAL, segments, ...) are decided by the serverless
/// manager.
///
/// # Example
///
/// ```no_run
/// use qdrant_client::serverless::{
///     CollectionConfig, DenseVectorConfig, Distance, KeywordIndex, QdrantServerless,
/// };
/// use qdrant_client::qdrant::{PointStruct, QueryPointsBuilder, UpsertPointsBuilder};
/// use qdrant_client::Payload;
///
/// # async fn run() -> Result<(), qdrant_client::QdrantError> {
/// let client = QdrantServerless::from_url("https://serverless.example.cloud.qdrant.io")
///     .api_key("<your api key>")
///     .build()?;
///
/// client
///     .create_collection(
///         "my-collection",
///         CollectionConfig::new()
///             .dense_vector(DenseVectorConfig::new(4, Distance::Cosine))
///             .payload_index("color", KeywordIndex),
///     )
///     .await?;
///
/// client
///     .upsert_points(UpsertPointsBuilder::new(
///         "my-collection",
///         vec![PointStruct::new(1, vec![0.1, 0.2, 0.3, 0.4], Payload::default())],
///     ))
///     .await?;
///
/// client
///     .query(QueryPointsBuilder::new("my-collection").query(vec![0.1, 0.2, 0.3, 0.4]))
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct QdrantServerless {
    points: Qdrant,
}

/// Builder for [`QdrantServerless`].
///
/// Defaults differ from the regular [`Qdrant`] client:
/// - compatibility checks are skipped (serverless has no matching server version)
/// - URLs without an explicit port default to port [`DEFAULT_SERVERLESS_GRPC_PORT`] (443)
#[derive(Clone)]
pub struct QdrantServerlessBuilder {
    config: QdrantConfig,
}

impl QdrantServerless {
    /// Start configuring a serverless client from a base URL.
    ///
    /// ```no_run
    /// use qdrant_client::serverless::QdrantServerless;
    ///
    /// # fn main() -> Result<(), qdrant_client::QdrantError> {
    /// let client = QdrantServerless::from_url("https://serverless.example.cloud.qdrant.io")
    ///     .api_key(std::env::var("QDRANT_API_KEY"))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_url(url: &str) -> QdrantServerlessBuilder {
        QdrantServerlessBuilder {
            config: QdrantConfig {
                uri: normalize_serverless_url(url),
                check_compatibility: false,
                ..QdrantConfig::default()
            },
        }
    }

    /// Create a serverless client from an existing config.
    ///
    /// Compatibility checks are forced off. Prefer [`Self::from_url`].
    pub fn new(mut config: QdrantConfig) -> QdrantResult<Self> {
        config.check_compatibility = false;
        config.uri = normalize_serverless_url(&config.uri);
        Ok(Self {
            points: Qdrant::new(config)?,
        })
    }

    async fn with_collections_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(CollectionsServiceClient<InterceptedService<Channel, MetadataInterceptor>>) -> O,
    ) -> QdrantResult<T> {
        let result = self
            .points
            .channel()
            .with_channel(
                |channel| {
                    let service = self.points.with_api_key(channel);
                    let mut client = CollectionsServiceClient::new(service)
                        .max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.points.config.compression {
                        client = client
                            .send_compressed(compression.into())
                            .accept_compressed(compression.into());
                    }
                    f(client)
                },
                true,
            )
            .await?;
        Ok(result)
    }
}

/// # Construct and connect
impl QdrantServerlessBuilder {
    /// Set an optional API key (sent as `api-key` metadata).
    pub fn api_key(mut self, api_key: impl AsOptionApiKey) -> Self {
        self.config.api_key = api_key.api_key();
        self
    }

    /// Set the timeout for API requests.
    pub fn timeout(mut self, timeout: impl AsTimeout) -> Self {
        self.config.timeout = timeout.timeout();
        self
    }

    /// Set the connect timeout.
    pub fn connect_timeout(mut self, timeout: impl AsTimeout) -> Self {
        self.config.connect_timeout = timeout.timeout();
        self
    }

    /// Set optional request compression.
    pub fn compression(mut self, compression: Option<CompressionEncoding>) -> Self {
        self.config.compression = compression;
        self
    }

    /// Add a custom header to send with every request.
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.custom_headers.push((key.into(), value.into()));
        self
    }

    /// Keep idle connections alive.
    pub fn keep_alive_while_idle(mut self) -> Self {
        self.config.keep_alive_while_idle = true;
        self
    }

    /// Build the configured [`QdrantServerless`] client.
    pub fn build(self) -> QdrantResult<QdrantServerless> {
        QdrantServerless::new(self.config)
    }
}

/// # Collection operations
///
/// Simplified, tenant-facing collection management. Storage internals are not exposed.
impl QdrantServerless {
    /// Creates a collection with the given tenant-facing configuration.
    ///
    /// At least one dense or sparse vector is required. Unlike the regular client,
    /// no storage internals (quantization, WAL, segment number, ...) can be
    /// configured: the serverless manager decides those.
    ///
    /// Returns the outcome string from the service (e.g. `"created"`).
    pub async fn create_collection(
        &self,
        collection_name: impl Into<String>,
        config: CollectionConfig,
    ) -> QdrantResult<String> {
        let request = CreateCollectionRequest {
            collection_name: collection_name.into(),
            config: Some(collection_config_to_grpc(&config)),
        };
        let request = &request;
        self.with_collections_client(|mut api| async move {
            let response = api.create_collection(request.clone()).await?;
            Ok(response.into_inner().result)
        })
        .await
    }

    /// Deletes a collection and all of its data.
    ///
    /// Returns `true` if the collection existed and was deleted, `false` otherwise.
    pub async fn delete_collection(
        &self,
        collection_name: impl Into<String>,
    ) -> QdrantResult<bool> {
        let request = DeleteCollectionRequest {
            collection_name: collection_name.into(),
        };
        let request = &request;
        self.with_collections_client(|mut api| async move {
            let response = api.delete_collection(request.clone()).await?;
            Ok(response.into_inner().deleted)
        })
        .await
    }

    /// Returns a collection's configuration and stats.
    ///
    /// Unlike the regular client, does not error if the collection is missing:
    /// check the [`CollectionInfo::exists`] field of the result.
    pub async fn get_collection(
        &self,
        collection_name: impl Into<String>,
    ) -> QdrantResult<CollectionInfo> {
        let request = GetCollectionRequest {
            collection_name: collection_name.into(),
        };
        let request = &request;
        let response = self
            .with_collections_client(|mut api| async move {
                Ok(api.get_collection(request.clone()).await?.into_inner())
            })
            .await?;
        Ok(CollectionInfo {
            exists: response.exists,
            config: response
                .config
                .as_ref()
                .map(collection_config_from_grpc)
                .transpose()?,
            point_count: response.point_count,
        })
    }

    /// Checks whether a collection exists.
    pub async fn collection_exists(
        &self,
        collection_name: impl Into<String>,
    ) -> QdrantResult<bool> {
        Ok(self.get_collection(collection_name).await?.exists)
    }

    /// Lists the collections of the space.
    ///
    /// Returns summaries (name and eventually consistent point count), ordered by name.
    pub async fn list_collections(&self) -> QdrantResult<Vec<CollectionSummary>> {
        self.with_collections_client(|mut api| async move {
            let response = api
                .list_collections(ListCollectionsRequest {})
                .await?
                .into_inner();
            Ok(response
                .collections
                .into_iter()
                .map(|c| CollectionSummary {
                    collection_name: c.collection_name,
                    point_count: c.point_count,
                })
                .collect())
        })
        .await
    }
}

/// # Point operations
///
/// Same semantics as the regular client, minus parameters serverless does not
/// support. Unsupported fields on the request (`read_consistency`,
/// `shard_key_selector`, `ordering`, `lookup_from`, `with_lookup`,
/// `update_filter`, `update_mode`) are cleared before the RPC.
impl QdrantServerless {
    /// Query points in a collection.
    pub async fn query(&self, request: impl Into<QueryPoints>) -> QdrantResult<QueryResponse> {
        let mut request = request.into();
        sanitize_query_points(&mut request);
        self.points.query(request).await
    }

    /// Batch multiple point queries in a collection.
    pub async fn query_batch(
        &self,
        request: impl Into<QueryBatchPoints>,
    ) -> QdrantResult<QueryBatchResponse> {
        let mut request = request.into();
        request.read_consistency = None;
        for query in &mut request.query_points {
            sanitize_query_points(query);
        }
        self.points.query_batch(request).await
    }

    /// Query points and group results by a payload field.
    pub async fn query_groups(
        &self,
        request: impl Into<QueryPointGroups>,
    ) -> QdrantResult<QueryGroupsResponse> {
        let mut request = request.into();
        request.read_consistency = None;
        request.shard_key_selector = None;
        request.lookup_from = None;
        request.with_lookup = None;
        self.points.query_groups(request).await
    }

    /// Retrieve points by IDs.
    pub async fn get_points(&self, request: impl Into<GetPoints>) -> QdrantResult<GetResponse> {
        let mut request = request.into();
        request.read_consistency = None;
        request.shard_key_selector = None;
        self.points.get_points(request).await
    }

    /// Scroll over points, optionally filtered.
    pub async fn scroll(&self, request: impl Into<ScrollPoints>) -> QdrantResult<ScrollResponse> {
        let mut request = request.into();
        request.read_consistency = None;
        request.shard_key_selector = None;
        self.points.scroll(request).await
    }

    /// Count points in a collection.
    pub async fn count(&self, request: impl Into<CountPoints>) -> QdrantResult<CountResponse> {
        let mut request = request.into();
        request.read_consistency = None;
        request.shard_key_selector = None;
        self.points.count(request).await
    }

    /// Insert or update points.
    ///
    /// Clears `ordering`, `shard_key_selector`, `update_filter`, and `update_mode`.
    pub async fn upsert_points(
        &self,
        request: impl Into<UpsertPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let mut request = request.into();
        request.ordering = None;
        request.shard_key_selector = None;
        request.update_filter = None;
        request.update_mode = None;
        self.points.upsert_points(request).await
    }

    /// Update vectors of existing points.
    pub async fn update_vectors(
        &self,
        request: impl Into<UpdatePointVectors>,
    ) -> QdrantResult<PointsOperationResponse> {
        let mut request = request.into();
        request.ordering = None;
        request.shard_key_selector = None;
        request.update_filter = None;
        self.points.update_vectors(request).await
    }

    /// Delete named vectors from points.
    pub async fn delete_vectors(
        &self,
        request: impl Into<DeletePointVectors>,
    ) -> QdrantResult<PointsOperationResponse> {
        let mut request = request.into();
        request.ordering = None;
        request.shard_key_selector = None;
        self.points.delete_vectors(request).await
    }

    /// Delete points by selector.
    ///
    /// Prefer selecting by explicit IDs: serverless rejects filtered updates.
    pub async fn delete_points(
        &self,
        request: impl Into<DeletePoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let mut request = request.into();
        request.ordering = None;
        request.shard_key_selector = None;
        self.points.delete_points(request).await
    }

    /// Set (merge) payload on points.
    pub async fn set_payload(
        &self,
        request: impl Into<SetPayloadPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let mut request = request.into();
        request.ordering = None;
        request.shard_key_selector = None;
        self.points.set_payload(request).await
    }

    /// Overwrite the entire payload of points.
    pub async fn overwrite_payload(
        &self,
        request: impl Into<SetPayloadPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let mut request = request.into();
        request.ordering = None;
        request.shard_key_selector = None;
        self.points.overwrite_payload(request).await
    }

    /// Delete payload keys from points.
    pub async fn delete_payload(
        &self,
        request: impl Into<DeletePayloadPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let mut request = request.into();
        request.ordering = None;
        request.shard_key_selector = None;
        self.points.delete_payload(request).await
    }

    /// Clear the entire payload of points.
    pub async fn clear_payload(
        &self,
        request: impl Into<ClearPayloadPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let mut request = request.into();
        request.ordering = None;
        request.shard_key_selector = None;
        self.points.clear_payload(request).await
    }

    /// Batch point update operations.
    ///
    /// Operations with filter-based selectors are rejected by the serverless service.
    pub async fn batch_update_points(
        &self,
        request: impl Into<UpdateBatchPoints>,
    ) -> QdrantResult<UpdateBatchResponse> {
        let mut request = request.into();
        request.ordering = None;
        self.points.update_points_batch(request).await
    }
}

fn sanitize_query_points(request: &mut QueryPoints) {
    request.read_consistency = None;
    request.shard_key_selector = None;
    request.lookup_from = None;
}

/// Ensure a serverless URL uses port 443 when none is specified.
fn normalize_serverless_url(url: &str) -> String {
    match url.parse::<Uri>() {
        Ok(uri) => {
            if uri.port().is_some() {
                return url.to_string();
            }
            let scheme = uri.scheme_str().unwrap_or("https");
            let Some(authority) = uri.authority() else {
                return url.to_string();
            };
            let host = authority.host();
            let path_and_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("");
            // Avoid duplicating "/" path when the URI has only the default path.
            let path = if path_and_query == "/" {
                ""
            } else {
                path_and_query
            };
            format!("{scheme}://{host}:{DEFAULT_SERVERLESS_GRPC_PORT}{path}")
        }
        Err(_) => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_adds_default_port() {
        assert_eq!(
            normalize_serverless_url("https://serverless.example.qdrant.io"),
            "https://serverless.example.qdrant.io:443"
        );
        assert_eq!(
            normalize_serverless_url("https://serverless.example.qdrant.io:8443"),
            "https://serverless.example.qdrant.io:8443"
        );
    }

    #[test]
    fn client_construction_is_offline() {
        let client = QdrantServerless::from_url("https://serverless.example.qdrant.io")
            .api_key("secret")
            .build()
            .unwrap();
        assert_eq!(
            client.points.config.uri,
            "https://serverless.example.qdrant.io:443"
        );
        assert_eq!(client.points.config.api_key.as_deref(), Some("secret"));
        assert!(!client.points.config.check_compatibility);
    }
}
