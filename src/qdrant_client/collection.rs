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
use crate::qdrant_client::{Qdrant, Result};

impl Qdrant {
    pub(crate) async fn with_collections_client<
        T,
        O: Future<Output = std::result::Result<T, Status>>,
    >(
        &self,
        f: impl Fn(CollectionsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T> {
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
    ) -> Result<CollectionOperationResponse> {
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
    ) -> Result<CollectionOperationResponse> {
        let create_collection = request.into();
        let create_collection_ref = &create_collection;
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.create(create_collection_ref.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn list_collections(&self) -> Result<ListCollectionsResponse> {
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.list(ListCollectionsRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn collection_exists(
        &self,
        request: impl Into<CollectionExistsRequest>,
    ) -> Result<bool> {
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

    pub async fn update_collection(
        &self,
        request: impl Into<UpdateCollection>,
    ) -> Result<CollectionOperationResponse> {
        let request = &request.into();

        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.update(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn collection_info(
        &self,
        request: impl Into<GetCollectionInfoRequest>,
    ) -> Result<GetCollectionInfoResponse> {
        let request = &request.into();
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.get(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn create_alias(
        &self,
        request: impl Into<CreateAlias>,
    ) -> Result<CollectionOperationResponse> {
        self.update_aliases(request.into()).await
    }

    pub async fn delete_alias(
        &self,
        request: impl Into<DeleteAlias>,
    ) -> Result<CollectionOperationResponse> {
        self.update_aliases(request.into()).await
    }

    pub async fn rename_alias(
        &self,
        request: impl Into<RenameAlias>,
    ) -> Result<CollectionOperationResponse> {
        self.update_aliases(request.into()).await
    }

    pub async fn update_aliases(
        &self,
        change_aliases: impl Into<alias_operations::Action> + Clone,
    ) -> Result<CollectionOperationResponse> {
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

    pub async fn list_collection_aliases(
        &self,
        request: impl Into<ListCollectionAliasesRequest>,
    ) -> Result<ListAliasesResponse> {
        let request = &request.into();
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api
                .list_collection_aliases(request.clone())
                .await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn list_aliases(&self) -> Result<ListAliasesResponse> {
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api.list_aliases(ListAliasesRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn collection_cluster_info(
        &self,
        request: impl Into<CollectionClusterInfoRequest>,
    ) -> Result<CollectionClusterInfoResponse> {
        let request = &request.into();
        self.with_collections_client(|mut collection_api| async move {
            let result = collection_api
                .collection_cluster_info(request.clone())
                .await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn update_collection_cluster_setup(
        &self,
        request: impl Into<UpdateCollectionClusterSetupRequest>,
    ) -> Result<UpdateCollectionClusterSetupResponse> {
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
    use super::*;
    use crate::payload::Payload;
    use crate::prelude::Distance;
    use crate::qdrant::{
        CountPointsBuilder, CreateCollectionBuilder, PointStruct, SearchPointsBuilder,
        UpsertPointsBuilder, VectorParamsBuilder,
    };
    use std::time::Duration;
    use tokio::time::sleep;
    use crate::qdrant_client::config::QdrantConfig;

    #[tokio::test]
    async fn create_collection_and_do_the_search() -> Result<()> {
        let config = QdrantConfig::from_url("http://localhost:6334");
        let client = Qdrant::new(Some(config))?;

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
