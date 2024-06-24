use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::client::QdrantClient;
use crate::qdrant::alias_operations::Action;
use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::update_collection_cluster_setup_request::Operation;
use crate::qdrant::{
    shard_key, AliasOperations, ChangeAliases, CollectionClusterInfoRequest,
    CollectionClusterInfoResponse, CollectionExistsRequest, CollectionOperationResponse,
    CollectionParamsDiff, CreateAlias, CreateCollection, CreateShardKey, CreateShardKeyRequest,
    CreateShardKeyResponse, DeleteAlias, DeleteCollection, DeleteShardKey, DeleteShardKeyRequest,
    DeleteShardKeyResponse, GetCollectionInfoRequest, GetCollectionInfoResponse, HnswConfigDiff,
    ListAliasesRequest, ListAliasesResponse, ListCollectionAliasesRequest, ListCollectionsRequest,
    ListCollectionsResponse, OptimizersConfigDiff, QuantizationConfigDiff, RenameAlias, ShardKey,
    SparseVectorConfig, UpdateCollection, UpdateCollectionClusterSetupRequest,
    UpdateCollectionClusterSetupResponse, VectorsConfigDiff,
};

impl QdrantClient {
    // Access to raw collection API
    pub async fn with_collections_client<T, O: Future<Output = anyhow::Result<T, Status>>>(
        &self,
        f: impl Fn(CollectionsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> anyhow::Result<T, Status> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::list_collections` instead"
    )]
    pub async fn list_collections(&self) -> anyhow::Result<ListCollectionsResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.list(ListCollectionsRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.8.0",
        note = "use new `qdrant_client::Qdrant::collection_exists` instead"
    )]
    pub async fn has_collection(&self, collection_name: impl ToString) -> anyhow::Result<bool> {
        let collection_name = collection_name.to_string();
        let response = self.list_collections().await?;
        let result = response
            .collections
            .into_iter()
            .any(|c| c.name == collection_name);

        Ok(result)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::collection_exists` instead"
    )]
    pub async fn collection_exists(&self, collection_name: impl ToString) -> anyhow::Result<bool> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::create_collection` instead"
    )]
    pub async fn create_collection(
        &self,
        details: &CreateCollection,
    ) -> anyhow::Result<CollectionOperationResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.create(details.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[allow(clippy::too_many_arguments)]
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_collection` instead"
    )]
    pub async fn update_collection(
        &self,
        collection_name: impl ToString,
        optimizers_config: Option<&OptimizersConfigDiff>,
        params: Option<&CollectionParamsDiff>,
        sparse_vectors_config: Option<&SparseVectorConfig>,
        hnsw_config: Option<&HnswConfigDiff>,
        vectors_config: Option<&VectorsConfigDiff>,
        quantization_config: Option<&QuantizationConfigDiff>,
    ) -> anyhow::Result<CollectionOperationResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_collection` instead"
    )]
    pub async fn delete_collection(
        &self,
        collection_name: impl ToString,
    ) -> anyhow::Result<CollectionOperationResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::collection_info` instead"
    )]
    pub async fn collection_info(
        &self,
        collection_name: impl ToString,
    ) -> anyhow::Result<GetCollectionInfoResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::create_alias` instead"
    )]
    pub async fn create_alias(
        &self,
        collection_name: impl ToString,
        alias_name: impl ToString,
    ) -> anyhow::Result<CollectionOperationResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_alias` instead"
    )]
    pub async fn delete_alias(
        &self,
        alias_name: impl ToString,
    ) -> anyhow::Result<CollectionOperationResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::rename_alias` instead"
    )]
    pub async fn rename_alias(
        &self,
        old_alias_name: impl ToString,
        new_alias_name: impl ToString,
    ) -> anyhow::Result<CollectionOperationResponse> {
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
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::create_alias`, `qdrant_client::Qdrant::rename_alias` or `qdrant_client::Qdrant::delete_alias` instead"
    )]
    pub async fn update_aliases(
        &self,
        change_aliases: ChangeAliases,
    ) -> anyhow::Result<CollectionOperationResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::list_collection_aliases` instead"
    )]
    pub async fn list_collection_aliases(
        &self,
        collection_name: impl ToString,
    ) -> anyhow::Result<ListAliasesResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::list_aliases` instead"
    )]
    pub async fn list_aliases(&self) -> anyhow::Result<ListAliasesResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.list_aliases(ListAliasesRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::collection_cluster_info` instead"
    )]
    pub async fn collection_cluster_info(
        &self,
        collection_name: impl ToString,
    ) -> anyhow::Result<CollectionClusterInfoResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::create_shard_key` instead"
    )]
    pub async fn create_shard_key(
        &self,
        collection_name: impl AsRef<str>,
        shard_key: &shard_key::Key,
        shards_number: Option<u32>,
        replication_factor: Option<u32>,
        placement: &[u64],
    ) -> anyhow::Result<CreateShardKeyResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::create_shard_key` instead"
    )]
    pub async fn delete_shard_key(
        &self,
        collection_name: impl AsRef<str>,
        shard_key: &shard_key::Key,
    ) -> anyhow::Result<DeleteShardKeyResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_collection_cluster_setup` instead"
    )]
    pub async fn update_collection_cluster_setup(
        &self,
        collection_name: impl ToString,
        operation: Operation,
    ) -> anyhow::Result<UpdateCollectionClusterSetupResponse> {
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
}
