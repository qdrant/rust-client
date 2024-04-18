pub mod collection;
pub mod config;

use crate::channel_pool::ChannelPool;
use crate::client::TokenInterceptor;
use crate::new_client::config::{ClientConfig, ClientConfigBuilder};
use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::{ListCollectionsRequest, ListCollectionsResponse};
use anyhow::Result;
use std::future::Future;
use std::sync::Arc;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Uri};
use tonic::Status;

use self::collection::NewCollectionBuilder;

#[derive(Clone)]
pub struct QdrantClient {
    pub channel: Arc<ChannelPool>,
    pub cfg: ClientConfig,
}

impl QdrantClient {
    /// Creates a new Qdrant client with the default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new Qdrant client with a default configuration and a custom url.
    pub fn from_url(url: impl ToString) -> Result<Self> {
        let config = ClientConfigBuilder::default().uri(url.to_string()).build();
        Self::with_config(config)
    }

    /// Creates a new Qdrant client with a custom config. A [`ClientConfig`] can be built using
    /// either `ClientConfig::builder()` or `ClientConfigBuilder::default()`.
    pub fn with_config(config: impl Into<ClientConfig>) -> Result<Self> {
        let cfg = config.into();

        let channel = ChannelPool::new(
            cfg.uri.parse::<Uri>()?,
            cfg.timeout,
            cfg.connect_timeout,
            cfg.keep_alive_while_idle,
        );

        Ok(Self {
            channel: Arc::new(channel),
            cfg,
        })
    }

    /// Wraps a channel with a token interceptor
    fn service_with_api_key(
        &self,
        channel: Channel,
    ) -> InterceptedService<Channel, TokenInterceptor> {
        let interceptor = TokenInterceptor::new(self.cfg.api_key.clone());
        InterceptedService::new(channel, interceptor)
    }

    // Access to raw collection API
    pub async fn with_collections_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(CollectionsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    let service = self.service_with_api_key(channel);
                    let client = CollectionsClient::new(service);
                    let client = client.max_decoding_message_size(usize::MAX);
                    f(client)
                },
                false,
            )
            .await
    }

    pub async fn list_collections(&self) -> Result<ListCollectionsResponse> {
        let res = self
            .with_collections_client(|mut ca| async move {
                let res = ca.list(ListCollectionsRequest {}).await?;
                Ok(res.into_inner())
            })
            .await?;
        Ok(res)
    }

    /// Create a new collection. Returns a builder that can be used to configure the new collection.
    ///
    /// # Example
    /// ```rust,no_run
    ///  use qdrant_client::new_client::collection::VectorsConfigBuilder;
    ///  use qdrant_client::new_client::QdrantClient;
    ///  use qdrant_client::qdrant::Distance;
    ///
    ///  let client = QdrantClient::new();
    ///  client
    ///     .create_collection("my_new_collection")
    ///     .vectors_config(VectorsConfigBuilder::new(768, Distance::Cosine).on_disk(true))
    ///     .on_disk_payload(true);
    /// ```
    pub fn create_collection(&self, name: impl ToString) -> NewCollectionBuilder {
        NewCollectionBuilder::new(self.clone(), name)
    }
}

impl Default for QdrantClient {
    fn default() -> Self {
        let config = ClientConfig::default();
        // The with_config method only fails if the provided uri is not parsable.
        // However the uri of the default ClientConfig is a constant in form of a valid uri so this
        // won't panic.
        Self::with_config(config).expect("Default uri is not parsable as uri")
    }
}
