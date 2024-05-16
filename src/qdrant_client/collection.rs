use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::prelude::{CreateCollection, DeleteCollection};
use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::{
    alias_operations, AliasOperations, ChangeAliases, CollectionClusterInfoRequest,
    CollectionClusterInfoResponse, CollectionExistsRequest, CollectionOperationResponse,
    CreateAlias, DeleteAlias, GetCollectionInfoRequest, GetCollectionInfoResponse,
    ListAliasesRequest, ListAliasesResponse, ListCollectionAliasesRequest, ListCollectionsRequest,
    ListCollectionsResponse, RenameAlias, UpdateCollection, UpdateCollectionClusterSetupRequest,
    UpdateCollectionClusterSetupResponse,
};
use crate::qdrant_client::errors::QdrantError;
use crate::qdrant_client::Qdrant;

impl Qdrant {
    pub(crate) async fn with_collections_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(CollectionsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, QdrantError> {
        let result = self
            .channel
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
            .await?;
        Ok(result)
    }

    pub async fn delete_collection(
        &self,
        request: impl Into<DeleteCollection>,
    ) -> Result<CollectionOperationResponse, QdrantError> {
        let delete_collection = &request.into();

        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.delete(delete_collection.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn create_collection(
        &self,
        request: impl Into<CreateCollection>,
    ) -> Result<CollectionOperationResponse, QdrantError> {
        let create_collection = request.into();
        let create_collection_ref = &create_collection;
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.create(create_collection_ref.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn list_collections(&self) -> anyhow::Result<ListCollectionsResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.list(ListCollectionsRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn collection_exists(
        &self,
        request: impl Into<CollectionExistsRequest>,
    ) -> anyhow::Result<bool> {
        let request = &request.into();
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.collection_exists(request.clone()).await?;
                Ok(result
                    .into_inner()
                    .result
                    .map(|r| r.exists)
                    .unwrap_or(false))
            })
            .await?)
    }

    pub async fn update_collection(
        &self,
        request: impl Into<UpdateCollection>,
    ) -> anyhow::Result<CollectionOperationResponse> {
        let request = &request.into();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.update(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn collection_info(
        &self,
        request: impl Into<GetCollectionInfoRequest>,
    ) -> anyhow::Result<GetCollectionInfoResponse> {
        let request = &request.into();
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.get(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn create_alias(
        &self,
        request: impl Into<CreateAlias>,
    ) -> anyhow::Result<CollectionOperationResponse> {
        self.update_aliases(request.into()).await
    }

    pub async fn delete_alias(
        &self,
        request: impl Into<DeleteAlias>,
    ) -> anyhow::Result<CollectionOperationResponse> {
        self.update_aliases(request.into()).await
    }

    pub async fn rename_alias(
        &self,
        request: impl Into<RenameAlias>,
    ) -> anyhow::Result<CollectionOperationResponse> {
        self.update_aliases(request.into()).await
    }

    pub async fn update_aliases(
        &self,
        change_aliases: impl Into<alias_operations::Action> + Clone,
    ) -> anyhow::Result<CollectionOperationResponse> {
        let action = change_aliases.into();
        let change = &ChangeAliases {
            actions: vec![AliasOperations {
                action: Some(action),
            }],
            timeout: None,
        };
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.update_aliases(change.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_collection_aliases(
        &self,
        request: impl Into<ListCollectionAliasesRequest>,
    ) -> anyhow::Result<ListAliasesResponse> {
        let request = &request.into();
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .list_collection_aliases(request.clone())
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_aliases(&self) -> anyhow::Result<ListAliasesResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.list_aliases(ListAliasesRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn collection_cluster_info(
        &self,
        request: impl Into<CollectionClusterInfoRequest>,
    ) -> anyhow::Result<CollectionClusterInfoResponse> {
        let request = &request.into();
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .collection_cluster_info(request.clone())
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn update_collection_cluster_setup(
        &self,
        request: impl Into<UpdateCollectionClusterSetupRequest>,
    ) -> anyhow::Result<UpdateCollectionClusterSetupResponse> {
        let request = &request.into();
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .update_collection_cluster_setup(request.clone())
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::QdrantClientConfig;
    use crate::qdrant::CreateCollectionBuilder;

    use super::*;

    #[tokio::test]
    async fn create_collection_and_do_the_search() {
        let config = QdrantClientConfig::from_url("http://localhost:6334");
        let client = Qdrant::new(Some(config)).unwrap();

        let collection_name = "test";

        client.delete_collection(collection_name).await.unwrap();

        let create_collection = CreateCollectionBuilder::new(collection_name);
        let _result = client.create_collection(create_collection).await.unwrap();
    }
}
