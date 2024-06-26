use crate::qdrant::{
    CreateShardKeyRequest, CreateShardKeyResponse, DeleteShardKeyRequest, DeleteShardKeyResponse,
};
use crate::qdrant_client::{Qdrant, QdrantResult};

/// # Sharding key operations
///
/// Create or delete shard keys for collections.
///
/// Documentation: <https://qdrant.tech/documentation/guides/distributed_deployment/#user-defined-sharding>
impl Qdrant {
    /// Create new shard key in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::shard_key::Key;
    /// use qdrant_client::qdrant::{CreateShardKeyBuilder, CreateShardKeyRequestBuilder};
    ///
    ///# async fn create_shard_key(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .create_shard_key(
    ///         CreateShardKeyRequestBuilder::new("my_collection").request(
    ///             CreateShardKeyBuilder::default()
    ///                 .shard_key(Key::Keyword("my_key".to_string())),
    ///         ),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/guides/distributed_deployment/#user-defined-sharding>
    pub async fn create_shard_key(
        &self,
        request: impl Into<CreateShardKeyRequest>,
    ) -> QdrantResult<CreateShardKeyResponse> {
        let request = &request.into();

        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.create_shard_key(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Delete existing shard key from a collection.
    ///
    /// Deleting a shard key destroys all shards and data placed in it.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::shard_key::Key;
    /// use qdrant_client::qdrant::DeleteShardKeyRequestBuilder;
    ///
    ///# async fn delete_shard_key(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_shard_key(
    ///         DeleteShardKeyRequestBuilder::new("my_collection")
    ///             .key(Key::Keyword("my_key".to_string())),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/guides/distributed_deployment/#user-defined-sharding>
    pub async fn delete_shard_key(
        &self,
        request: impl Into<DeleteShardKeyRequest>,
    ) -> QdrantResult<DeleteShardKeyResponse> {
        let request = &request.into();

        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.delete_shard_key(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }
}
