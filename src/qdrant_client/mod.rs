mod builers;
mod collection;
pub mod config;
mod conversions;
pub mod errors;
mod points;
mod query;
mod sharding_keys;
mod snapshot;

use crate::channel_pool::ChannelPool;
use crate::qdrant::{qdrant_client, HealthCheckReply, HealthCheckRequest};
use crate::Error;
use std::future::Future;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Uri};
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::qdrant_client::config::QdrantConfig;

/// [`Qdrant`] client result
pub type Result<T> = std::result::Result<T, Error>;

/// A builder for [`Qdrant`]
pub type QdrantBuilder = QdrantConfig;

/// Qdrant client
///
/// Connects to a Qdrant server and provides an API interface.
pub struct Qdrant {
    /// Client configuration
    pub config: QdrantConfig,

    /// Internal connection pool
    channel: ChannelPool,
}

impl Qdrant {
    /// Create a builder to setup the client
    pub fn from_url(url: &str) -> QdrantBuilder {
        QdrantBuilder::from_url(url)
    }

    pub fn new(config: Option<QdrantConfig>) -> Result<Self> {
        let config = config.unwrap_or_default();

        let channel = ChannelPool::new(
            config.uri.parse::<Uri>()?,
            config.timeout,
            config.connect_timeout,
            config.keep_alive_while_idle,
        );

        let client = Self { channel, config };

        Ok(client)
    }

    /// Wraps a channel with a token interceptor
    fn with_api_key(&self, channel: Channel) -> InterceptedService<Channel, TokenInterceptor> {
        let interceptor = TokenInterceptor::new(self.config.api_key.clone());
        InterceptedService::new(channel, interceptor)
    }

    // Access to raw root qdrant API
    async fn with_root_qdrant_client<T, O: Future<Output = std::result::Result<T, Status>>>(
        &self,
        f: impl Fn(qdrant_client::QdrantClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T> {
        let result = self
            .channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client = qdrant_client::QdrantClient::new(service)
                        .max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.config.compression {
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

    pub async fn health_check(&self) -> Result<HealthCheckReply> {
        self.with_root_qdrant_client(|mut qdrant_api| async move {
            let result = qdrant_api.health_check(HealthCheckRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }
}
