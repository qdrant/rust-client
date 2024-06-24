mod builders;
mod collection;
pub mod config;
mod conversions;
pub mod error;
mod points;
mod query;
mod sharding_keys;
mod snapshot;

use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Uri};
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::channel_pool::ChannelPool;
use crate::qdrant::{qdrant_client, HealthCheckReply, HealthCheckRequest};
use crate::qdrant_client::config::QdrantConfig;
use crate::QdrantError;

/// [`Qdrant`] client result
pub type QdrantResult<T> = Result<T, QdrantError>;

/// A builder for [`Qdrant`]
pub type QdrantBuilder = QdrantConfig;

/// Qdrant client
///
/// Connects to a Qdrant server and provides an API interface.
///
/// # Connect
///
/// Connect to a Qdrant instance with just an [URL](Qdrant::from_url):
///
/// ```no_run
/// use qdrant_client::Qdrant;
///
///# async fn connect() -> Result<(), qdrant_client::QdrantError> {
/// let client = Qdrant::from_url("http://localhost:6334").build()?;
///# Ok(())
///# }
/// ```
///
/// Connect to a Qdrant instance with an [URL](Qdrant::from_url),
/// [API key](QdrantBuilder::with_api_key) and [timeout](QdrantBuilder::with_timeout):
///
/// ```no_run
/// use qdrant_client::Qdrant;
///
///# async fn connect() -> Result<(), qdrant_client::QdrantError> {
/// let client = Qdrant::from_url("http://localhost:6334")
///     .with_api_key(std::env::var("QDRANT_API_KEY"))
///     .with_timeout(std::time::Duration::from_secs(10))
///     .build()?;
///# Ok(())
///# }
/// ```
///
/// # Operations
///
/// Common operations include:
///
/// - [`create_collection`](Qdrant::create_collection) - Create a new collection
/// - [`upsert_points`](Qdrant::upsert_points) - Insert or update points
/// - [`search_points`](Qdrant::search_points) - Search points with similarity search
/// - [All operations](Qdrant#implementations)
pub struct Qdrant {
    /// Client configuration
    pub config: QdrantConfig,

    /// Internal connection pool
    channel: ChannelPool,
}

/// Methods to construct a new Qdrant client.
impl Qdrant {
    /// Create a new Qdrant client.
    ///
    /// If no client client configuration is given the [default](QdrantConfig::default) is used.
    pub fn new(config: Option<QdrantConfig>) -> QdrantResult<Self> {
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

    /// Build a new Qdrant client with the given URL.
    ///
    /// ```no_run
    /// use qdrant_client::Qdrant;
    ///
    ///# async fn connect() -> Result<(), qdrant_client::QdrantError> {
    /// let client = Qdrant::from_url("http://localhost:6334").build()?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// See more ways to connect [here](Self#connect).
    pub fn from_url(url: &str) -> QdrantBuilder {
        QdrantBuilder::from_url(url)
    }

    /// Wraps a channel with a token interceptor
    fn with_api_key(&self, channel: Channel) -> InterceptedService<Channel, TokenInterceptor> {
        let interceptor = TokenInterceptor::new(self.config.api_key.clone());
        InterceptedService::new(channel, interceptor)
    }

    // Access to raw root qdrant API
    async fn with_root_qdrant_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(qdrant_client::QdrantClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> QdrantResult<T> {
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

    /// Health check.
    ///
    /// Do a health check and fetch server information such as the current version and commit.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn list_collections(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.health_check().await?;
    ///# Ok(())
    ///# }
    /// ```
    pub async fn health_check(&self) -> QdrantResult<HealthCheckReply> {
        self.with_root_qdrant_client(|mut qdrant_api| async move {
            let result = qdrant_api.health_check(HealthCheckRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }
}
