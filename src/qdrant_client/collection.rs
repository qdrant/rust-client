use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::prelude::{CreateCollection, DeleteCollection};
use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::CollectionOperationResponse;
use crate::qdrant_client::errors::QdrantError;
use crate::qdrant_client::Qdrant;

impl Qdrant {
    async fn with_collections_client<T, O: Future<Output = Result<T, Status>>>(
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
