use std::time::Duration;

use crate::{Qdrant, QdrantError};

/// Qdrant client configuration
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
}

impl QdrantConfig {
    pub fn from_url(url: &str) -> Self {
        QdrantConfig {
            uri: url.to_string(),
            ..Self::default()
        }
    }

    /// Sets the API key or token
    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn set_connect_timeout(&mut self, connect_timeout: Duration) {
        self.connect_timeout = connect_timeout;
    }

    pub fn set_keep_alive_while_idle(&mut self, keep_alive_while_idle: bool) {
        self.keep_alive_while_idle = keep_alive_while_idle;
    }

    pub fn set_compression(&mut self, compression: Option<CompressionEncoding>) {
        self.compression = compression;
    }

    /// set the API key, builder-like. The API key argument can be any of
    /// `&str`, `String`, `Option<&str>`, `Option<String>` or `Result<String>`.
    ///
    /// # Examples:
    ///
    /// A typical use case might be getting the key from an env var:
    ///
    /// ```rust,no_run
    /// use qdrant_client::Qdrant;
    ///
    /// let client = Qdrant::from_url("http://localhost:6334")
    ///     .with_api_key(std::env::var("QDRANT_API_KEY"))
    ///     .build();
    /// ```
    ///
    /// Another possibility might be getting it out of some config
    ///
    /// ```rust,no_run
    ///# use std::collections::HashMap;
    /// use qdrant_client::Qdrant;
    ///# let config: HashMap<&str, String> = HashMap::new();
    /// let client = Qdrant::from_url("http://localhost:6334")
    ///     .with_api_key(config.get("api_key"))
    ///     .build();
    /// ```
    pub fn with_api_key(mut self, api_key: impl MaybeApiKey) -> Self {
        self.api_key = api_key.maybe_key();
        self
    }

    /// Configure the service to keep the connection alive while idle
    pub fn keep_alive_while_idle(mut self) -> Self {
        self.keep_alive_while_idle = true;
        self
    }

    /// Set the timeout for this client
    pub fn with_timeout(mut self, timeout: impl AsTimeout) -> Self {
        self.timeout = timeout.timeout();
        self
    }

    /// Set the connect timeout for this client
    pub fn with_connect_timeout(mut self, timeout: impl AsTimeout) -> Self {
        self.connect_timeout = timeout.timeout();
        self
    }

    /// Set the compression to use for this client
    pub fn with_compression(mut self, compression: Option<CompressionEncoding>) -> Self {
        self.compression = compression;
        self
    }

    /// Build the Qdrant
    pub fn build(self) -> Result<Qdrant, QdrantError> {
        Qdrant::new(Some(self))
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
///     .with_timeout(10)
///     .with_timeout(Duration::from_secs(10));
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
///     .with_api_key("secret")
///     .with_api_key(String::from("secret"))
///     .with_api_key(std::env::var("QDRANT_API_KEY"))
///     .with_api_key(None::<String>);
/// ```
pub trait MaybeApiKey {
    fn maybe_key(self) -> Option<String>;
}

impl MaybeApiKey for &str {
    fn maybe_key(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl MaybeApiKey for String {
    fn maybe_key(self) -> Option<String> {
        Some(self)
    }
}

impl MaybeApiKey for Option<String> {
    fn maybe_key(self) -> Option<String> {
        self
    }
}

impl MaybeApiKey for Option<&String> {
    fn maybe_key(self) -> Option<String> {
        self.map(ToOwned::to_owned)
    }
}

impl MaybeApiKey for Option<&str> {
    fn maybe_key(self) -> Option<String> {
        self.map(ToOwned::to_owned)
    }
}

impl<E: Sized> MaybeApiKey for Result<String, E> {
    fn maybe_key(self) -> Option<String> {
        self.ok()
    }
}
