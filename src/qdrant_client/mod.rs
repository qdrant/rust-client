pub mod collection;
pub mod errors;
mod points;

use crate::channel_pool::ChannelPool;
use crate::qdrant::{qdrant_client, HealthCheckReply, HealthCheckRequest};
use std::future::Future;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Uri};
use tonic::Status;

pub use crate::auth::TokenInterceptor;
pub use crate::config::{AsTimeout, CompressionEncoding, MaybeApiKey, QdrantClientConfig};
pub use crate::payload::Payload;
use crate::qdrant_client::errors::QdrantError;

/// A builder type for `QdrantClient`s
pub type QdrantClientBuilder = QdrantClientConfig;

pub struct Qdrant {
    pub channel: ChannelPool,
    pub cfg: QdrantClientConfig,
}

impl Qdrant {
    /// Create a builder to setup the client
    pub fn from_url(url: &str) -> QdrantClientBuilder {
        QdrantClientBuilder::from_url(url)
    }

    pub fn new(cfg: Option<QdrantClientConfig>) -> Result<Self, QdrantError> {
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
    async fn with_root_qdrant_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(qdrant_client::QdrantClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, QdrantError> {
        let result = self
            .channel
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
            .await?;
        Ok(result)
    }

    pub async fn health_check(&self) -> Result<HealthCheckReply, QdrantError> {
        self.with_root_qdrant_client(|mut qdrant_api| async move {
            let result = qdrant_api.health_check(HealthCheckRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }
}
