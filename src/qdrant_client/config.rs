use std::time::Duration;

use crate::{Qdrant, QdrantError};

/// Qdrant client configuration
///
/// The client is normally constructed through [`Qdrant::from_url`](crate::Qdrant::from_url):
///
/// ```rust,no_run
/// use qdrant_client::Qdrant;
/// use qdrant_client::config::CompressionEncoding;
///
/// let client = Qdrant::from_url("http://localhost:6334")
///     .api_key(std::env::var("QDRANT_API_KEY"))
///     .timeout(std::time::Duration::from_secs(10))
///     .compression(Some(CompressionEncoding::Gzip))
///     .build();
/// ```
#[derive(Clone)]
pub struct QdrantConfig {
    /// Qdrant server URI to connect to
    pub uri: String,

    /// Timeout for API requests
    pub timeout: Duration,

    /// Timeout for connecting to the Qdrant server
    pub connect_timeout: Duration,

    /// Whether to keep idle connections active
    pub keep_alive_while_idle: bool,

    /// Optional API key or token to use for authorization
    pub api_key: Option<String>,

    /// Optional compression schema to use for API requests
    pub compression: Option<CompressionEncoding>,

    /// Whether to check compatibility between the client and server versions
    pub check_compatibility: bool,

    /// Amount of concurrent connections.
    pub pool_size: usize,
}

impl QdrantConfig {
    /// Start configuring a Qdrant client with an URL
    ///
    /// ```rust,no_run
    ///# use qdrant_client::config::QdrantConfig;
    /// let client = QdrantConfig::from_url("http://localhost:6334").build();
    /// ```
    ///
    /// This is normally done through [`Qdrant::from_url`](crate::Qdrant::from_url).
    pub fn from_url(url: &str) -> Self {
        QdrantConfig {
            uri: url.to_string(),
            ..Self::default()
        }
    }

    /// Set an optional API key
    ///
    /// # Examples
    ///
    /// A typical use case might be getting the key from an environment variable:
    ///
    /// ```rust,no_run
    /// use qdrant_client::Qdrant;
    ///
    /// let client = Qdrant::from_url("http://localhost:6334")
    ///     .api_key(std::env::var("QDRANT_API_KEY"))
    ///     .build();
    /// ```
    ///
    /// Or you might get it from some configuration:
    ///
    /// ```rust,no_run
    ///# use std::collections::HashMap;
    ///# let config: HashMap<&str, String> = HashMap::new();
    ///# use qdrant_client::Qdrant;
    /// let client = Qdrant::from_url("http://localhost:6334")
    ///     .api_key(config.get("api_key"))
    ///     .build();
    /// ```
    pub fn api_key(mut self, api_key: impl AsOptionApiKey) -> Self {
        self.api_key = api_key.api_key();
        self
    }

    /// Keep the connection alive while idle
    pub fn keep_alive_while_idle(mut self) -> Self {
        self.keep_alive_while_idle = true;
        self
    }

    /// Set the timeout for this client
    ///
    /// ```rust,no_run
    /// use qdrant_client::Qdrant;
    ///
    /// let client = Qdrant::from_url("http://localhost:6334")
    ///     .timeout(std::time::Duration::from_secs(10))
    ///     .build();
    /// ```
    pub fn timeout(mut self, timeout: impl AsTimeout) -> Self {
        self.timeout = timeout.timeout();
        self
    }

    /// Set the connect timeout for this client
    ///
    /// ```rust,no_run
    /// use qdrant_client::Qdrant;
    ///
    /// let client = Qdrant::from_url("http://localhost:6334")
    ///     .connect_timeout(std::time::Duration::from_secs(10))
    ///     .build();
    /// ```
    pub fn connect_timeout(mut self, timeout: impl AsTimeout) -> Self {
        self.connect_timeout = timeout.timeout();
        self
    }

    /// Set the compression to use for this client
    ///
    /// ```rust,no_run
    /// use qdrant_client::Qdrant;
    /// use qdrant_client::config::CompressionEncoding;
    ///
    /// let client = Qdrant::from_url("http://localhost:6334")
    ///     .compression(Some(CompressionEncoding::Gzip))
    ///     .build();
    /// ```
    pub fn compression(mut self, compression: Option<CompressionEncoding>) -> Self {
        self.compression = compression;
        self
    }

    /// Set an API key
    ///
    /// Also see [`api_key()`](fn@Self::api_key).
    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }

    /// Set the timeout for this client
    ///
    /// Also see [`timeout()`](fn@Self::timeout).
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Set the connection timeout for this client
    ///
    /// Also see [`connect_timeout()`](fn@Self::connect_timeout).
    pub fn set_connect_timeout(&mut self, connect_timeout: Duration) {
        self.connect_timeout = connect_timeout;
    }

    /// Set whether to keep the connection alive when idle
    ///
    /// Also see [`keep_alive_while_idle()`](fn@Self::keep_alive_while_idle).
    pub fn set_keep_alive_while_idle(&mut self, keep_alive_while_idle: bool) {
        self.keep_alive_while_idle = keep_alive_while_idle;
    }

    /// Set the compression to use for this client
    ///
    /// Also see [`compression()`](fn@Self::compression).
    pub fn set_compression(&mut self, compression: Option<CompressionEncoding>) {
        self.compression = compression;
    }

    /// Build the configured [`Qdrant`] client
    pub fn build(self) -> Result<Qdrant, QdrantError> {
        Qdrant::new(self)
    }

    pub fn skip_compatibility_check(mut self) -> Self {
        self.check_compatibility = false;
        self
    }

    /// Set the pool size of concurrent connections.
    pub fn set_pool_size(&mut self, pool_size: usize) {
        self.pool_size = pool_size;
    }
}

/// Default Qdrant client configuration.
///
/// Connects to `http://localhost:6334` without an API key.
impl Default for QdrantConfig {
    fn default() -> Self {
        Self {
            uri: String::from("http://localhost:6334"),
            timeout: Duration::from_secs(5),
            connect_timeout: Duration::from_secs(5),
            keep_alive_while_idle: true,
            api_key: None,
            compression: None,
            check_compatibility: true,
            pool_size: 1,
        }
    }
}

/// Type of compression to use for requests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionEncoding {
    Gzip,
}

impl From<CompressionEncoding> for tonic::codec::CompressionEncoding {
    fn from(encoding: CompressionEncoding) -> Self {
        match encoding {
            CompressionEncoding::Gzip => tonic::codec::CompressionEncoding::Gzip,
        }
    }
}

/// Set a timeout from various types
///
/// For example:
///
/// ```rust
///# use std::time::Duration;
///# use qdrant_client::Qdrant;
///# let mut config = Qdrant::from_url("http://localhost:6334");
/// config
///     .timeout(10)
///     .timeout(Duration::from_secs(10));
/// ```
pub trait AsTimeout {
    fn timeout(self) -> Duration;
}

impl AsTimeout for Duration {
    fn timeout(self) -> Duration {
        self
    }
}

impl AsTimeout for u64 {
    fn timeout(self) -> Duration {
        Duration::from_secs(self)
    }
}

/// Set an optional API key from various types
///
/// For example:
///
/// ```rust
///# use std::time::Duration;
///# use qdrant_client::Qdrant;
///# let mut config = Qdrant::from_url("http://localhost:6334");
/// config
///     .api_key("secret")
///     .api_key(String::from("secret"))
///     .api_key(std::env::var("QDRANT_API_KEY"))
///     .api_key(None::<String>);
/// ```
pub trait AsOptionApiKey {
    fn api_key(self) -> Option<String>;
}

impl AsOptionApiKey for &str {
    fn api_key(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl AsOptionApiKey for String {
    fn api_key(self) -> Option<String> {
        Some(self)
    }
}

impl AsOptionApiKey for Option<String> {
    fn api_key(self) -> Option<String> {
        self
    }
}

impl AsOptionApiKey for Option<&String> {
    fn api_key(self) -> Option<String> {
        self.map(ToOwned::to_owned)
    }
}

impl AsOptionApiKey for Option<&str> {
    fn api_key(self) -> Option<String> {
        self.map(ToOwned::to_owned)
    }
}

impl<E: Sized> AsOptionApiKey for Result<String, E> {
    fn api_key(self) -> Option<String> {
        self.ok()
    }
}
