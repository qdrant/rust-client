use crate::client::QdrantClient;
use crate::prelude::CreateCollection;
use crate::qdrant::CollectionOperationResponse;
use tonic::async_trait;

#[async_trait]
pub trait ApiCallable {
    type Response;

    async fn exec(&self, client: &mut QdrantClient) -> anyhow::Result<Self::Response>;
}

#[async_trait]
impl ApiCallable for CreateCollection {
    type Response = CollectionOperationResponse;

    async fn exec(&self, client: &mut QdrantClient) -> anyhow::Result<Self::Response> {
        client.create_collection(self).await
    }
}
