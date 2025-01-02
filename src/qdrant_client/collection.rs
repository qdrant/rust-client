use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::{
    alias_operations, AliasOperations, ChangeAliases, CollectionClusterInfoRequest,
    CollectionClusterInfoResponse, CollectionExistsRequest, CollectionOperationResponse,
    CreateAlias, CreateCollection, DeleteAlias, DeleteCollection, GetCollectionInfoRequest,
    GetCollectionInfoResponse, ListAliasesRequest, ListAliasesResponse,
    ListCollectionAliasesRequest, ListCollectionsRequest, ListCollectionsResponse, RenameAlias,
    UpdateCollection, UpdateCollectionClusterSetupRequest, UpdateCollectionClusterSetupResponse,
};
use crate::qdrant_client::version_check::is_compatible;
use crate::qdrant_client::{Qdrant, QdrantResult};

/// # Collection operations
///
/// Create, update and delete collections, manage collection aliases and collection cluster
/// configuration.
///
/// Documentation: <https://qdrant.tech/documentation/concepts/collections/>
impl Qdrant {
    pub(super) async fn with_collections_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(CollectionsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> QdrantResult<T> {
        if self.config.check_compatibility && self.is_compatible().is_none() {
            let client_version = env!("CARGO_PKG_VERSION").to_string();
            let server_version = match self.health_check().await {
                Ok(info) => info.version,
                Err(_) => "Unknown".to_string(),
            };
            if server_version == "Unknown" {
                println!(
                    "Failed to obtain server version. \
                Unable to check client-server compatibility. \
                Set check_compatibility=false to skip version check."
                );
            } else {
                let is_compatible = is_compatible(Some(&client_version), Some(&server_version));
                self.set_is_compatible(Some(is_compatible));
                println!("Client version {client_version} is not compatible with server version {server_version}. \
                Major versions should match and minor version difference must not exceed 1. \
                Set check_compatibility=false to skip version check.");
            }
        }

        let result = self
            .channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client =
                        CollectionsClient::new(service).max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.config.compression {
                        client = client
                            .send_compressed(compression.into())
                            .accept_compressed(compression.into());
                    }
                    f(client)
                },
                false,
            )
            .await?;
        Ok(result)
    }

    /// Create a new collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};
    ///
    ///# async fn create_collection(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .create_collection(
    ///         CreateCollectionBuilder::new("my_collection")
    ///             .vectors_config(VectorParamsBuilder::new(100, Distance::Cosine)),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#create-a-collection>
    pub async fn create_collection(
        &self,
        request: impl Into<CreateCollection>,
    ) -> QdrantResult<CollectionOperationResponse> {
        let create_collection = request.into();
        let create_collection_ref = &create_collection;
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.create(create_collection_ref.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Get collection info.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn collection_info(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.collection_info("my_collection").await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#collection-info>
    pub async fn collection_info(
        &self,
        request: impl Into<GetCollectionInfoRequest>,
    ) -> QdrantResult<GetCollectionInfoResponse> {
        let request = &request.into();
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.get(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// List all collections.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn list_collections(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.list_collections().await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// This only lists collection names. To list collection name aliases, use
    /// [`list_aliases`](Self::list_aliases).
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#list-all-collections>
    pub async fn list_collections(&self) -> QdrantResult<ListCollectionsResponse> {
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.list(ListCollectionsRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Check whether a collection exists.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn collection_exists(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.collection_exists("my_collection").await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#check-collection-existence>
    #[doc(alias = "has_collection")]
    pub async fn collection_exists(
        &self,
        request: impl Into<CollectionExistsRequest>,
    ) -> QdrantResult<bool> {
        let request = &request.into();
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.collection_exists(request.clone()).await?;
            Ok(result
                .into_inner()
                .result
                .map(|r| r.exists)
                .unwrap_or(false))
        })
        .await
    }

    /// Update collection.
    ///
    /// Change parameters of a collection, such as the indexing threshold, for a collection that
    /// has already been created.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{OptimizersConfigDiffBuilder, UpdateCollectionBuilder};
    ///
    ///# async fn create_collection(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .update_collection(
    ///         UpdateCollectionBuilder::new("my_collection").optimizers_config(
    ///             OptimizersConfigDiffBuilder::default().indexing_threshold(10_000),
    ///         ),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#update-collection-parameters>
    pub async fn update_collection(
        &self,
        request: impl Into<UpdateCollection>,
    ) -> QdrantResult<CollectionOperationResponse> {
        let request = &request.into();

        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.update(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Delete an existing collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn delete_collection(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.delete_collection("my_collection").await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#delete-collection>
    pub async fn delete_collection(
        &self,
        request: impl Into<DeleteCollection>,
    ) -> QdrantResult<CollectionOperationResponse> {
        let delete_collection = &request.into();

        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.delete(delete_collection.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Create new collection name alias.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::CreateAliasBuilder;
    ///
    ///# async fn create_alias(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .create_alias(CreateAliasBuilder::new("my_collection", "my_alias"))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#create-alias>
    pub async fn create_alias(
        &self,
        request: impl Into<CreateAlias>,
    ) -> QdrantResult<CollectionOperationResponse> {
        self.update_aliases(request.into()).await
    }

    /// List collection name aliases for all collections.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn list_aliases(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.list_aliases().await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// This only lists collection name aliases. To list collection names, use
    /// [`list_collections`](Self::list_collections).
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#list-all-aliases>
    pub async fn list_aliases(&self) -> QdrantResult<ListAliasesResponse> {
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.list_aliases(ListAliasesRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// List collection name aliases for a specific collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn list_collection_aliases(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.list_collection_aliases("my_collection").await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#list-collection-aliases>
    pub async fn list_collection_aliases(
        &self,
        request: impl Into<ListCollectionAliasesRequest>,
    ) -> QdrantResult<ListAliasesResponse> {
        let request = &request.into();
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api
                .list_collection_aliases(request.clone())
                .await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Rename existing collection name alias.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::RenameAliasBuilder;
    ///
    ///# async fn rename_alias(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .rename_alias(RenameAliasBuilder::new("old_alias", "new_alias"))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub async fn rename_alias(
        &self,
        request: impl Into<RenameAlias>,
    ) -> QdrantResult<CollectionOperationResponse> {
        self.update_aliases(request.into()).await
    }

    /// Update a collection name alias.
    ///
    /// Create, rename or delete a collection alias.
    ///
    /// To do this, use:
    ///
    /// - [`create_alias`](Self::create_alias)
    /// - [`rename_alias`](Self::rename_alias)
    /// - [`delete_alias`](Self::delete_alias)
    async fn update_aliases(
        &self,
        change_aliases: impl Into<alias_operations::Action> + Clone,
    ) -> QdrantResult<CollectionOperationResponse> {
        let action = change_aliases.into();
        let change = &ChangeAliases {
            actions: vec![AliasOperations {
                action: Some(action),
            }],
            timeout: None,
        };
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.update_aliases(change.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Delete existing collection name alias.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::CreateAliasBuilder;
    ///
    ///# async fn delete_alias(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_alias("my_alias")
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#remove-alias>
    pub async fn delete_alias(
        &self,
        request: impl Into<DeleteAlias>,
    ) -> QdrantResult<CollectionOperationResponse> {
        self.update_aliases(request.into()).await
    }

    /// List cluster info of a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn collection_cluster_info(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.collection_cluster_info("my_collection").await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#collection-info>
    pub async fn collection_cluster_info(
        &self,
        request: impl Into<CollectionClusterInfoRequest>,
    ) -> QdrantResult<CollectionClusterInfoResponse> {
        let request = &request.into();
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api
                .collection_cluster_info(request.clone())
                .await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Update collection cluster setup.
    ///
    /// Perform a collection cluster
    /// [`Operation`](crate::qdrant::update_collection_cluster_setup_request::Operation), such as
    /// [`MoveShard`](crate::qdrant::MoveShard), [`ReplicateShard`](crate::qdrant::ReplicateShard)
    /// or [`CreateShardKey`](crate::qdrant::CreateShardKey).
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{MoveShardBuilder, UpdateCollectionClusterSetupRequestBuilder};
    ///
    ///# async fn update_collection_cluster_setup(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .update_collection_cluster_setup(UpdateCollectionClusterSetupRequestBuilder::new(
    ///         "my_collection",
    ///         MoveShardBuilder::new(
    ///             0, // Shard ID
    ///             0, // From peer ID
    ///             1, // To peer ID
    ///         ),
    ///     ))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/collections/#create-a-collection>
    pub async fn update_collection_cluster_setup(
        &self,
        request: impl Into<UpdateCollectionClusterSetupRequest>,
    ) -> QdrantResult<UpdateCollectionClusterSetupResponse> {
        let request = &request.into();
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api
                .update_collection_cluster_setup(request.clone())
                .await?;
            Ok(result.into_inner())
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;

    use super::*;
    use crate::builders::CreateCollectionBuilder;
    use crate::payload::Payload;
    use crate::qdrant::{
        CountPointsBuilder, Distance, PointStruct, SearchPointsBuilder, UpsertPointsBuilder,
        VectorParamsBuilder,
    };

    #[tokio::test]
    async fn create_collection_and_do_the_search() -> QdrantResult<()> {
        let client = Qdrant::from_url("http://localhost:6334").build()?;

        let collection_name = "test2";

        let _ = client.delete_collection(collection_name).await;

        assert!(!client.collection_exists(collection_name).await?);

        let create_collection = CreateCollectionBuilder::new(collection_name)
            .vectors_config(VectorParamsBuilder::new(3, Distance::Cosine));
        let _result = client.create_collection(create_collection).await?;

        sleep(Duration::from_secs(1)).await;

        assert!(client.collection_exists(collection_name).await?);

        client
            .upsert_points(
                UpsertPointsBuilder::new(
                    collection_name,
                    vec![
                        PointStruct::new(0, vec![1.0, 0.0, 0.0], Payload::default()),
                        PointStruct::new(1, vec![0.0, 1.0, 0.0], Payload::default()),
                    ],
                )
                .wait(true),
            )
            .await?;

        let count = client
            .count(CountPointsBuilder::new(collection_name))
            .await?;

        assert_eq!(count.result.unwrap().count, 2);

        let _search_res = client
            .search_points(SearchPointsBuilder::new(
                collection_name,
                vec![0.8, 0.0, 0.0],
                2,
            ))
            .await?;

        Ok(())
    }
}
