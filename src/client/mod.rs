mod api_callable;
pub mod collection;
pub mod points;
pub mod snapshot;

use crate::channel_pool::ChannelPool;
use crate::qdrant::{qdrant_client, HealthCheckReply, HealthCheckRequest};
use anyhow::Result;
use std::future::Future;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Uri};
use tonic::Status;

pub use crate::auth::TokenInterceptor;
use crate::client::api_callable::ApiCallable;
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

    /// Wraps a channel with a token interceptor
    fn with_api_key(&self, channel: Channel) -> InterceptedService<Channel, TokenInterceptor> {
        let interceptor = TokenInterceptor::new(self.cfg.api_key.clone());
        InterceptedService::new(channel, interceptor)
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

    pub async fn health_check(&self) -> Result<HealthCheckReply> {
        Ok(self
            .with_root_qdrant_client(|mut qdrant_api| async move {
                let result = qdrant_api.health_check(HealthCheckRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    /// Executes the passed API request and returns it's result. This is the same as calling
    /// request.exe(&mut client).
    pub async fn exec<R, A>(&mut self, request: A) -> Result<R>
    where
        A: ApiCallable<Response = R>,
    {
        request.exec(self).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Distance;
    use crate::qdrant::{CreateCollectionBuilder, HnswConfigDiffBuilder, VectorParamsBuilder};

    const TEST_COLLECTION: &str = "my_test_collection_1234";

    #[tokio::test]
    async fn create_collection() -> Result<()> {
        let config = QdrantClientConfig::from_url("http://localhost:6334");
        let mut client = QdrantClient::new(Some(config))?;

        if client.collection_exists(TEST_COLLECTION).await? {
            client.delete_collection(TEST_COLLECTION).await?;
        }

        let create_collection = CreateCollectionBuilder::default()
            .collection_name(TEST_COLLECTION)
            .vectors_config(
                VectorParamsBuilder::new(768, Distance::Cosine)
                    .hnsw_config(HnswConfigDiffBuilder::default().on_disk(true)),
            )
            .build();

        create_collection.exec(&mut client).await?;
        // client.exec(create_collection).await?;  <= works too

        Ok(())
    }
}
