#![allow(deprecated)]

pub mod collection;
#[deprecated(
    since = "1.10.0",
    note = "use new config types at `qdrant_client::config` instead"
)]
mod config;
pub mod points;
pub mod snapshot;

use std::future::Future;

use anyhow::Result;
pub use config::{AsTimeout, CompressionEncoding, MaybeApiKey, QdrantClientConfig};
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Uri};
use tonic::Status;

pub use crate::auth::TokenInterceptor;
use crate::channel_pool::ChannelPool;
pub use crate::payload::Payload;
use crate::qdrant::{qdrant_client, HealthCheckReply, HealthCheckRequest};

/// A builder for `QdrantClient`s
#[deprecated(since = "1.10.0", note = "use `qdrant_client::QdrantBuilder` instead")]
pub type QdrantClientBuilder = QdrantClientConfig;

/// Deprecated Qdrant client
///
/// # Deprecated
///
/// This client is deprecated.
///
/// Please switch to the new [`Qdrant`](crate::Qdrant) client. It is easier to use and provides a
/// more robust interface.
///
/// See examples at the [crate root](crate) or at each individual [`Qdrant`](crate::Qdrant)
/// operation.
#[deprecated(since = "1.10.0", note = "use `qdrant_client::Qdrant` instead")]
pub struct QdrantClient {
    pub channel: ChannelPool,
    pub cfg: QdrantClientConfig,
}

impl QdrantClient {
    /// Create a builder to setup the client
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::from_url` instead"
    )]
    pub fn from_url(url: &str) -> QdrantClientBuilder {
        QdrantClientBuilder::from_url(url)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::new` instead"
    )]
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::health_check` instead"
    )]
    pub async fn health_check(&self) -> Result<HealthCheckReply> {
        Ok(self
            .with_root_qdrant_client(|mut qdrant_api| async move {
                let result = qdrant_api.health_check(HealthCheckRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }
}
