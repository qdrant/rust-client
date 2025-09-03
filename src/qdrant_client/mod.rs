pub(crate) mod builders;
mod collection;
pub mod config;
mod conversions;
pub mod error;
mod index;
mod payload;
mod points;
mod query;
mod search;
mod sharding_keys;
mod snapshot;
mod version_check;

use std::future::Future;
use std::sync::Arc;
use std::thread;

use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Uri};
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::channel_pool::ChannelPool;
use crate::qdrant::{qdrant_client, HealthCheckReply, HealthCheckRequest};
use crate::qdrant_client::config::QdrantConfig;
use crate::qdrant_client::version_check::is_compatible;
use crate::QdrantError;

/// [`Qdrant`] client result
pub type QdrantResult<T> = Result<T, QdrantError>;

/// A builder for [`Qdrant`]
pub type QdrantBuilder = QdrantConfig;

/// API client to interact with a [Qdrant](https://qdrant.tech/) server.
///
/// Connects to a Qdrant server and provides an API interface.
///
/// # Set up
///
/// Set up a [`Qdrant`] client to connect to a Qdrant instance with just an [URL](Qdrant::from_url):
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
/// Or use an [URL](Qdrant::from_url), [API key](fn@QdrantBuilder::api_key) and
/// [timeout](fn@QdrantBuilder::timeout):
///
/// ```no_run
/// use qdrant_client::Qdrant;
///
///# async fn connect() -> Result<(), qdrant_client::QdrantError> {
/// let client = Qdrant::from_url("http://localhost:6334")
///     .api_key(std::env::var("QDRANT_API_KEY"))
///     .timeout(std::time::Duration::from_secs(10))
///     .build()?;
///# Ok(())
///# }
/// ```
///
/// # Operations
///
/// Categories:
///
/// - [Collection operations](Self#collection-operations) - manage collections, aliases and cluster configuration
/// - [Point operations](Self#point-operations) - manage points and vectors
/// - [Payload operations](Self#payload-operations) - manage point payloads
/// - [Search operations](Self#search-operations) - search and explore points
/// - [Query operations](Self#query-operations) - query points using universal search
/// - [Index operations](Self#index-operations) - manage field and payload indices
/// - [Snapshot operations](Self#snapshot-operations) - manage instance or collection snapshots
/// - [Shard key operations](Self#sharding-key-operations) - manage shard keys
///
/// Common operations include:
///
/// - [`create_collection`](Self::create_collection) - create a new collection
/// - [`upsert_points`](Self::upsert_points) - insert or update points
/// - [`search_points`](Self::search_points) - search points with similarity search
#[derive(Clone)]
pub struct Qdrant {
    /// Client configuration
    pub config: QdrantConfig,

    /// Internal connection pool
    channel: Arc<ChannelPool>,
}

/// # Construct and connect
///
/// Methods to construct a new Qdrant client.
impl Qdrant {
    /// Create a new Qdrant client.
    ///
    /// Constructs the client and connects based on the given [`QdrantConfig`](config::QdrantConfig).
    pub fn new(config: QdrantConfig) -> QdrantResult<Self> {
        if config.check_compatibility {
            // create a temporary client to check compatibility
            let channel = ChannelPool::new(
                config.uri.parse::<Uri>()?,
                config.timeout,
                config.connect_timeout,
                config.keep_alive_while_idle,
                config.pool_size,
            );
            let client = Self {
                channel: Arc::new(channel),
                config: config.clone(),
            };

            // We're in sync context, spawn temporary runtime in thread to do async health check
            let server_version = thread::scope(|s| {
                s.spawn(|| {
                    tokio::runtime::Builder::new_current_thread()
                        .enable_io()
                        .enable_time()
                        .build()
                        .map_err(QdrantError::Io)?
                        .block_on(client.health_check())
                })
                .join()
                .expect("Failed to join health check thread")
            })
            .ok()
            .map(|info| info.version);

            let client_version = env!("CARGO_PKG_VERSION").to_string();
            if let Some(server_version) = server_version {
                let is_compatible = is_compatible(Some(&client_version), Some(&server_version));
                if !is_compatible {
                    println!("Client version {client_version} is not compatible with server version {server_version}. \
                    Major versions should match and minor version difference must not exceed 1. \
                    Set check_compatibility=false to skip version check.");
                }
            } else {
                println!(
                    "Failed to obtain server version. \
                    Unable to check client-server compatibility. \
                    Set check_compatibility=false to skip version check."
                );
            }
        }

        let channel = ChannelPool::new(
            config.uri.parse::<Uri>()?,
            config.timeout,
            config.connect_timeout,
            config.keep_alive_while_idle,
            config.pool_size,
        );

        let client = Self {
            channel: Arc::new(channel),
            config,
        };

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
    /// See more ways to set up the client [here](Self#set-up).
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
