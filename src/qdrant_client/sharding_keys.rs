use crate::qdrant::{
    CreateShardKeyRequest, CreateShardKeyResponse, DeleteShardKeyRequest, DeleteShardKeyResponse,
};
use crate::qdrant_client::Qdrant;

impl Qdrant {
    pub async fn create_shard_key(
        &self,
        request: impl Into<CreateShardKeyRequest>,
    ) -> anyhow::Result<CreateShardKeyResponse> {
        let request = &request.into();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.create_shard_key(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_shard_key(
        &self,
        request: impl Into<DeleteShardKeyRequest>,
    ) -> anyhow::Result<DeleteShardKeyResponse> {
        let request = &request.into();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.delete_shard_key(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }
}
