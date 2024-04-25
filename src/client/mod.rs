pub mod snapshot;

use crate::channel_pool::ChannelPool;
use crate::qdrant::alias_operations::Action;
use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::update_collection_cluster_setup_request::Operation;
use crate::qdrant::{
    qdrant_client, shard_key, AliasOperations, ChangeAliases, ClearPayloadPoints,
    CollectionClusterInfoRequest, CollectionClusterInfoResponse, CollectionExistsRequest,
    CollectionOperationResponse, CollectionParamsDiff, CountPoints, CountResponse, CreateAlias,
    CreateCollection, CreateFieldIndexCollection, CreateShardKey, CreateShardKeyRequest,
    CreateShardKeyResponse, DeleteAlias, DeleteCollection, DeleteFieldIndexCollection,
    DeletePayloadPoints, DeletePointVectors, DeletePoints, DeleteShardKey, DeleteShardKeyRequest,
    DeleteShardKeyResponse, DiscoverBatchPoints, DiscoverBatchResponse, DiscoverPoints,
    DiscoverResponse, FieldType, GetCollectionInfoRequest, GetCollectionInfoResponse, GetPoints,
    GetResponse, HealthCheckReply, HealthCheckRequest, HnswConfigDiff, ListAliasesRequest,
    ListAliasesResponse, ListCollectionAliasesRequest, ListCollectionsRequest,
    ListCollectionsResponse, OptimizersConfigDiff, PayloadIndexParams, PointId, PointStruct,
    PointVectors, PointsOperationResponse, PointsSelector, PointsUpdateOperation,
    QuantizationConfigDiff, ReadConsistency, RecommendBatchPoints, RecommendBatchResponse,
    RecommendGroupsResponse, RecommendPointGroups, RecommendPoints, RecommendResponse, RenameAlias,
    ScrollPoints, ScrollResponse, SearchBatchPoints, SearchBatchResponse, SearchGroupsResponse,
    SearchPointGroups, SearchPoints, SearchResponse, SetPayloadPoints, ShardKey, ShardKeySelector,
    SparseVectorConfig, UpdateBatchPoints, UpdateBatchResponse, UpdateCollection,
    UpdateCollectionClusterSetupRequest, UpdateCollectionClusterSetupResponse, UpdatePointVectors,
    UpsertPoints, VectorsConfigDiff, VectorsSelector, WithPayloadSelector, WithVectorsSelector,
    WriteOrdering,
};
use anyhow::Result;
use std::future::Future;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Uri};
use tonic::Status;

pub use crate::auth::TokenInterceptor;
pub use crate::config::{AsTimeout, CompressionEncoding, MaybeApiKey, QdrantClientConfig};
pub use crate::payload::Payload;

/// A builder type for `QdrantClient`s
pub type QdrantClientBuilder = QdrantClientConfig;

pub struct QdrantClient {
    pub channel: ChannelPool,
    pub cfg: QdrantClientConfig,
}

impl QdrantClient {
    /// Create a builder to setup the client
    pub fn from_url(url: &str) -> QdrantClientBuilder {
        QdrantClientBuilder::from_url(url)
    }

    /// Wraps a channel with a token interceptor
    fn with_api_key(&self, channel: Channel) -> InterceptedService<Channel, TokenInterceptor> {
        let interceptor = TokenInterceptor::new(self.cfg.api_key.clone());
        InterceptedService::new(channel, interceptor)
    }

    // Access to raw collection API
    pub async fn with_collections_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(CollectionsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client =
                        CollectionsClient::new(service).max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.cfg.compression {
                        client = client
                            .send_compressed(compression.into())
                            .accept_compressed(compression.into());
                    }
                    f(client)
                },
                false,
            )
            .await
    }

    // Access to raw points API
    pub async fn with_points_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(PointsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client =
                        PointsClient::new(service).max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.cfg.compression {
                        client = client
                            .send_compressed(compression.into())
                            .accept_compressed(compression.into());
                    }
                    f(client)
                },
                true,
            )
            .await
    }

    // Access to raw root qdrant API
    pub async fn with_root_qdrant_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(qdrant_client::QdrantClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client = qdrant_client::QdrantClient::new(service)
                        .max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.cfg.compression {
                        client = client
                            .send_compressed(compression.into())
                            .accept_compressed(compression.into());
                    }
                    f(client)
                },
                true,
            )
            .await
    }

    pub fn new(cfg: Option<QdrantClientConfig>) -> Result<Self> {
        let cfg = cfg.unwrap_or_default();

        let channel = ChannelPool::new(
            cfg.uri.parse::<Uri>()?,
            cfg.timeout,
            cfg.connect_timeout,
            cfg.keep_alive_while_idle,
        );

        let client = Self { channel, cfg };

        Ok(client)
    }

    pub async fn health_check(&self) -> Result<HealthCheckReply> {
        Ok(self
            .with_root_qdrant_client(|mut qdrant_api| async move {
                let result = qdrant_api.health_check(HealthCheckRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_collections(&self) -> Result<ListCollectionsResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.list(ListCollectionsRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(since = "1.8.0", note = "Please use `collection_exists` instead")]
    pub async fn has_collection(&self, collection_name: impl ToString) -> Result<bool> {
        let collection_name = collection_name.to_string();
        let response = self.list_collections().await?;
        let result = response
            .collections
            .into_iter()
            .any(|c| c.name == collection_name);

        Ok(result)
    }

    pub async fn collection_exists(&self, collection_name: impl ToString) -> Result<bool> {
        let collection_name_ref = &collection_name.to_string();
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let request = CollectionExistsRequest {
                    collection_name: collection_name_ref.clone(),
                };
                let result = collection_api.collection_exists(request).await?;
                Ok(result
                    .into_inner()
                    .result
                    .map(|r| r.exists)
                    .unwrap_or(false))
            })
            .await?)
    }

    pub async fn create_collection(
        &self,
        details: &CreateCollection,
    ) -> Result<CollectionOperationResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.create(details.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_collection(
        &self,
        collection_name: impl ToString,
        optimizers_config: Option<&OptimizersConfigDiff>,
        params: Option<&CollectionParamsDiff>,
        sparse_vectors_config: Option<&SparseVectorConfig>,
        hnsw_config: Option<&HnswConfigDiff>,
        vectors_config: Option<&VectorsConfigDiff>,
        quantization_config: Option<&QuantizationConfigDiff>,
    ) -> Result<CollectionOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .update(UpdateCollection {
                        collection_name: collection_name_ref.to_string(),
                        optimizers_config: optimizers_config.cloned(),
                        timeout: None,
                        params: params.cloned(),
                        sparse_vectors_config: sparse_vectors_config.cloned(),
                        hnsw_config: hnsw_config.cloned(),
                        vectors_config: vectors_config.cloned(),
                        quantization_config: quantization_config.cloned(),
                    })
                    .await?;

                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_collection(
        &self,
        collection_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .delete(DeleteCollection {
                        collection_name: collection_name_ref.to_string(),
                        ..Default::default()
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn collection_info(
        &self,
        collection_name: impl ToString,
    ) -> Result<GetCollectionInfoResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .get(GetCollectionInfoRequest {
                        collection_name: collection_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn create_alias(
        &self,
        collection_name: impl ToString,
        alias_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let create_alias = CreateAlias {
            collection_name: collection_name.to_string(),
            alias_name: alias_name.to_string(),
        };
        let change_aliases = ChangeAliases {
            actions: vec![AliasOperations {
                action: Some(Action::CreateAlias(create_alias)),
            }],
            timeout: None,
        };
        self.update_aliases(change_aliases).await
    }

    pub async fn delete_alias(
        &self,
        alias_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let delete_alias = DeleteAlias {
            alias_name: alias_name.to_string(),
        };
        let change_aliases = ChangeAliases {
            actions: vec![AliasOperations {
                action: Some(Action::DeleteAlias(delete_alias)),
            }],
            timeout: None,
        };
        self.update_aliases(change_aliases).await
    }

    pub async fn rename_alias(
        &self,
        old_alias_name: impl ToString,
        new_alias_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let rename_alias = RenameAlias {
            old_alias_name: old_alias_name.to_string(),
            new_alias_name: new_alias_name.to_string(),
        };
        let change_aliases = ChangeAliases {
            actions: vec![AliasOperations {
                action: Some(Action::RenameAlias(rename_alias)),
            }],
            timeout: None,
        };
        self.update_aliases(change_aliases).await
    }

    // lower level API
    pub async fn update_aliases(
        &self,
        change_aliases: ChangeAliases,
    ) -> Result<CollectionOperationResponse> {
        let change_aliases = change_aliases.clone();
        let chang_aliases_ref = &change_aliases;
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .update_aliases(chang_aliases_ref.clone())
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_collection_aliases(
        &self,
        collection_name: impl ToString,
    ) -> Result<ListAliasesResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .list_collection_aliases(ListCollectionAliasesRequest {
                        collection_name: collection_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_aliases(&self) -> Result<ListAliasesResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.list_aliases(ListAliasesRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn collection_cluster_info(
        &self,
        collection_name: impl ToString,
    ) -> Result<CollectionClusterInfoResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let request = CollectionClusterInfoRequest {
                    collection_name: collection_name_ref.to_string(),
                };
                let result = collection_api.collection_cluster_info(request).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn create_shard_key(
        &self,
        collection_name: impl AsRef<str>,
        shard_key: &shard_key::Key,
        shards_number: Option<u32>,
        replication_factor: Option<u32>,
        placement: &[u64],
    ) -> Result<CreateShardKeyResponse> {
        let collection_name = collection_name.as_ref();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .create_shard_key(CreateShardKeyRequest {
                        collection_name: collection_name.to_string(),
                        request: Some(CreateShardKey {
                            shard_key: Some(ShardKey::from(shard_key.clone())),
                            shards_number,
                            replication_factor,
                            placement: placement.to_vec(),
                        }),
                        timeout: None,
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_shard_key(
        &self,
        collection_name: impl AsRef<str>,
        shard_key: &shard_key::Key,
    ) -> Result<DeleteShardKeyResponse> {
        let collection_name = collection_name.as_ref();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .delete_shard_key(DeleteShardKeyRequest {
                        collection_name: collection_name.to_string(),
                        request: Some(DeleteShardKey {
                            shard_key: Some(ShardKey::from(shard_key.clone())),
                        }),
                        timeout: None,
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn update_collection_cluster_setup(
        &self,
        collection_name: impl ToString,
        operation: Operation,
    ) -> Result<UpdateCollectionClusterSetupResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let operation_ref = &operation;
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let request = UpdateCollectionClusterSetupRequest {
                    collection_name: collection_name_ref.to_string(),
                    timeout: None,
                    operation: Some(operation_ref.clone()),
                };
                let result = collection_api
                    .update_collection_cluster_setup(request)
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    async fn _batch_updates(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
        wait: bool,
    ) -> Result<UpdateBatchResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        Ok(self
            .with_points_client(|mut points_api| async move {
                Ok(points_api
                    .update_batch(UpdateBatchPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(wait),
                        operations: operations.to_vec(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?
                    .into_inner())
            })
            .await?)
    }

    pub async fn batch_updates(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> Result<UpdateBatchResponse> {
        self._batch_updates(collection_name, operations, ordering, false)
            .await
    }

    pub async fn batch_updates_blocking(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> Result<UpdateBatchResponse> {
        self._batch_updates(collection_name, operations, ordering, true)
            .await
    }

    /// Update or insert points into the collection.
    /// If points with given ID already exist, they will be overwritten.
    /// This method does *not* wait for completion of the operation, use
    /// [`upsert_points_blocking`] for that.
    /// Also this method does not split the points to insert to avoid timeouts,
    /// look at [`upsert_points_batch`] for that.
    pub async fn upsert_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._upsert_points(
            collection_name,
            shard_key_selector,
            &points,
            false,
            ordering,
        )
        .await
    }

    /// Update or insert points into the collection, wait for completion.
    /// If points with given ID already exist, they will be overwritten.
    /// This method does not split the points to insert to avoid timeouts,
    /// look at [`upsert_points_batch`] for that.
    pub async fn upsert_points_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._upsert_points(collection_name, shard_key_selector, &points, true, ordering)
            .await
    }

    #[inline]
    async fn _upsert_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointStruct],
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;
        Ok(self
            .with_points_client(|mut points_api| async move {
                Ok(points_api
                    .upsert(UpsertPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        points: points.to_vec(),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?
                    .into_inner())
            })
            .await?)
    }

    /// Update or insert points into the collection, splitting in chunks.
    /// If points with given ID already exist, they will be overwritten.
    /// This method does *not* wait for completion of the operation, use
    /// [`upsert_points_batch_blocking`] for that.
    pub async fn upsert_points_batch(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
        chunk_size: usize,
    ) -> Result<PointsOperationResponse> {
        self._upsert_points_batch(
            collection_name,
            shard_key_selector,
            &points,
            false,
            ordering,
            chunk_size,
        )
        .await
    }

    /// Update or insert points into the collection, splitting in chunks and
    /// waiting for completion of each.
    /// If points with given ID already exist, they will be overwritten.
    pub async fn upsert_points_batch_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
        chunk_size: usize,
    ) -> Result<PointsOperationResponse> {
        self._upsert_points_batch(
            collection_name,
            shard_key_selector,
            &points,
            true,
            ordering,
            chunk_size,
        )
        .await
    }

    #[inline]
    async fn _upsert_points_batch(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointStruct],
        block: bool,
        ordering: Option<WriteOrdering>,
        chunk_size: usize,
    ) -> Result<PointsOperationResponse> {
        if points.len() < chunk_size {
            return self
                ._upsert_points(collection_name, shard_key_selector, points, block, ordering)
                .await;
        }
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;
        Ok(self
            .with_points_client(|mut points_api| async move {
                let mut resp = PointsOperationResponse {
                    result: None,
                    time: 0.0,
                };
                for chunk in points.chunks(chunk_size) {
                    let PointsOperationResponse { result, time } = points_api
                        .upsert(UpsertPoints {
                            collection_name: collection_name_ref.to_string(),
                            wait: Some(block),
                            points: chunk.to_vec(),
                            ordering: ordering_ref.cloned(),
                            shard_key_selector: shard_keys_ref.clone(),
                        })
                        .await?
                        .into_inner();
                    resp.result = result;
                    resp.time += time;
                }
                Ok(resp)
            })
            .await?)
    }

    pub async fn set_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: Payload,
        payload_key: Option<String>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._set_payload(
            collection_name,
            shard_key_selector,
            points,
            &payload,
            payload_key,
            false,
            ordering,
        )
        .await
    }

    pub async fn set_payload_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: Payload,
        payload_key: Option<String>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._set_payload(
            collection_name,
            shard_key_selector,
            points,
            &payload,
            payload_key,
            true,
            ordering,
        )
        .await
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    async fn _set_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: &Payload,
        payload_key: Option<String>,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;
        let payload_key_ref = payload_key.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .set_payload(SetPayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        payload: payload.0.clone(),
                        points_selector: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                        key: payload_key_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn overwrite_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: Payload,
        payload_key: Option<String>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._overwrite_payload(
            collection_name,
            shard_key_selector,
            points,
            &payload,
            payload_key,
            false,
            ordering,
        )
        .await
    }

    pub async fn overwrite_payload_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: Payload,
        payload_key: Option<String>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._overwrite_payload(
            collection_name,
            shard_key_selector,
            points,
            &payload,
            payload_key,
            true,
            ordering,
        )
        .await
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    async fn _overwrite_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: &Payload,
        payload_key: Option<String>,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;
        let payload_key_ref = payload_key.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .overwrite_payload(SetPayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        payload: payload.0.clone(),
                        points_selector: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                        key: payload_key_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        keys: Vec<String>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_payload(
            collection_name,
            shard_key_selector,
            points,
            &keys,
            false,
            ordering,
        )
        .await
    }

    pub async fn delete_payload_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        keys: Vec<String>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_payload(
            collection_name,
            shard_key_selector,
            points,
            &keys,
            true,
            ordering,
        )
        .await
    }

    #[inline]
    async fn _delete_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        keys: &[String],
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .delete_payload(DeletePayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        keys: keys.to_owned(),
                        points_selector: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn clear_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: Option<PointsSelector>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._clear_payload(
            collection_name,
            shard_key_selector,
            points_selector.as_ref(),
            false,
            ordering,
        )
        .await
    }

    pub async fn clear_payload_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: Option<PointsSelector>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._clear_payload(
            collection_name,
            shard_key_selector,
            points_selector.as_ref(),
            true,
            ordering,
        )
        .await
    }

    #[inline]
    async fn _clear_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: Option<&PointsSelector>,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .clear_payload(ClearPayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        points: points_selector.cloned(),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn get_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointId],
        with_vectors: Option<impl Into<WithVectorsSelector>>,
        with_payload: Option<impl Into<WithPayloadSelector>>,
        read_consistency: Option<ReadConsistency>,
    ) -> Result<GetResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        let with_vectors = with_vectors.map(|v| v.into());
        let with_payload = with_payload.map(|v| v.into());

        let with_vectors_ref = with_vectors.as_ref();
        let with_payload_ref = with_payload.as_ref();
        let read_consistency_ref = read_consistency.as_ref();

        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .get(GetPoints {
                        collection_name: collection_name_ref.to_string(),
                        ids: points.to_owned(),
                        with_payload: with_payload_ref.cloned(),
                        with_vectors: with_vectors_ref.cloned(),
                        read_consistency: read_consistency_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;

                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn search_points(&self, request: &SearchPoints) -> Result<SearchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.search(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn search_batch_points(
        &self,
        request: &SearchBatchPoints,
    ) -> Result<SearchBatchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.search_batch(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn search_groups(&self, request: &SearchPointGroups) -> Result<SearchGroupsResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.search_groups(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_points(collection_name, shard_key_selector, false, points, ordering)
            .await
    }

    pub async fn delete_points_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_points(collection_name, shard_key_selector, true, points, ordering)
            .await
    }

    async fn _delete_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        blocking: bool,
        points: &PointsSelector,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .delete(DeletePoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(blocking),
                        points: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_vectors(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: &PointsSelector,
        vector_selector: &VectorsSelector,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_vectors(
            collection_name,
            shard_key_selector,
            false,
            points_selector,
            vector_selector,
            ordering,
        )
        .await
    }

    pub async fn delete_vectors_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: &PointsSelector,
        vector_selector: &VectorsSelector,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_vectors(
            collection_name,
            shard_key_selector,
            true,
            points_selector,
            vector_selector,
            ordering,
        )
        .await
    }

    async fn _delete_vectors(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        blocking: bool,
        points_selector: &PointsSelector,
        vector_selector: &VectorsSelector,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .delete_vectors(DeletePointVectors {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(blocking),
                        points_selector: Some(points_selector.clone()),
                        vectors: Some(vector_selector.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn update_vectors(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointVectors],
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._update_vectors(collection_name, shard_key_selector, false, points, ordering)
            .await
    }

    pub async fn update_vectors_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointVectors],
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._update_vectors(collection_name, shard_key_selector, true, points, ordering)
            .await
    }

    async fn _update_vectors(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        blocking: bool,
        points: &[PointVectors],
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .update_vectors(UpdatePointVectors {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(blocking),
                        points: points.to_owned(),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn scroll(&self, request: &ScrollPoints) -> Result<ScrollResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.scroll(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn recommend(&self, request: &RecommendPoints) -> Result<RecommendResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.recommend(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn recommend_batch(
        &self,
        request: &RecommendBatchPoints,
    ) -> Result<RecommendBatchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.recommend_batch(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn recommend_groups(
        &self,
        request: &RecommendPointGroups,
    ) -> Result<RecommendGroupsResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.recommend_groups(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn discover(&self, request: &DiscoverPoints) -> Result<DiscoverResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.discover(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn discover_batch(
        &self,
        request: &DiscoverBatchPoints,
    ) -> Result<DiscoverBatchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.discover_batch(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn count(&self, request: &CountPoints) -> Result<CountResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.count(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    /// Perform multiple point, vector and payload insert, update and delete operations in one request.
    /// This method does *not* wait for completion of the operation, use
    /// [`update_batch_blocking`] for that.
    pub async fn update_batch_points(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> Result<UpdateBatchResponse> {
        self._update_batch_points(collection_name, false, operations, ordering)
            .await
    }

    /// Perform multiple point, vector and payload insert, update and delete operations in one request.
    /// This method waits for completion on each operation.
    pub async fn update_batch_points_blocking(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> Result<UpdateBatchResponse> {
        self._update_batch_points(collection_name, true, operations, ordering)
            .await
    }

    async fn _update_batch_points(
        &self,
        collection_name: impl ToString,
        blocking: bool,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> Result<UpdateBatchResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .update_batch(UpdateBatchPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(blocking),
                        operations: operations.to_owned(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    /// Create index for a payload field
    pub async fn _create_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
        field_index_params: Option<&PayloadIndexParams>,
        wait: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let field_name = field_name.to_string();
        let field_name_ref = field_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut client| async move {
                let result = client
                    .create_field_index(CreateFieldIndexCollection {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(wait),
                        field_name: field_name_ref.to_string(),
                        field_type: Some(field_type.into()),
                        field_index_params: field_index_params.cloned(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn create_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
        field_index_params: Option<&PayloadIndexParams>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._create_field_index(
            collection_name,
            field_name,
            field_type,
            field_index_params,
            false,
            ordering,
        )
        .await
    }

    pub async fn create_field_index_blocking(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
        field_index_params: Option<&PayloadIndexParams>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._create_field_index(
            collection_name,
            field_name,
            field_type,
            field_index_params,
            true,
            ordering,
        )
        .await
    }

    pub async fn _delete_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        wait: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let field_name = field_name.to_string();
        let field_name_ref = field_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut client| async move {
                let result = client
                    .delete_field_index(DeleteFieldIndexCollection {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(wait),
                        field_name: field_name_ref.to_string(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_field_index(collection_name, field_name, false, ordering)
            .await
    }

    pub async fn delete_field_index_blocking(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_field_index(collection_name, field_name, true, ordering)
            .await
    }
}
