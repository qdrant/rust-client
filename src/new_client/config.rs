use crate::client::MaybeApiKey;
use derive_builder::Builder;
use std::time::Duration;

pub const DEFAULT_URI: &str = "http://localhost:6334";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
pub const DEFAULT_KEEP_ALIVE_WHILE_IDLE: bool = true;

/// Configuration for the Qdrant client.
#[derive(Clone, Debug, Builder)]
#[builder(build_fn(private, error = "std::convert::Infallible", name = "build_inner"))]
pub struct ClientConfig {
    /// The URI of the Qdrant instance to connect to.
    #[builder(default = "String::from(DEFAULT_URI)", setter(into))]
    pub uri: String,

    #[builder(default = "DEFAULT_TIMEOUT")]
    pub timeout: Duration,

    #[builder(default = "DEFAULT_CONNECT_TIMEOUT")]
    pub connect_timeout: Duration,

    #[builder(default = "DEFAULT_KEEP_ALIVE_WHILE_IDLE")]
    pub keep_alive_while_idle: bool,

    #[builder(default, setter(custom))]
    pub api_key: Option<String>,
}

impl ClientConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfigBuilder::default().build()
    }
}

impl From<ClientConfigBuilder> for ClientConfig {
    fn from(mut value: ClientConfigBuilder) -> Self {
        value.build()
    }
}

impl ClientConfigBuilder {
    pub fn build(&mut self) -> ClientConfig {
        self.build_inner().unwrap()
    }

    /// Sets the API key that should be used by the client. This can be anything that
    /// implements `MaybeApiKey`, so any of `&str`, `String`, `Option<&str>`, `Option<String>`
    /// or `Result<String>`.
    ///
    /// # Examples:
    ///
    /// A typical use case might be getting the key from an env var:
    /// ```rust, no_run
    /// use qdrant_client::new_client::config::ClientConfig;
    ///
    /// let client = ClientConfig::builder().uri("localhost:6334")
    ///     .api_key(std::env::var("QDRANT_API_KEY"))
    ///     .build();
    /// ```
    /// Another possibility might be getting it out of some config
    /// ```rust, no_run
    /// use qdrant_client::new_client::config::ClientConfig;
    ///# use std::collections::HashMap;
    ///# let config: HashMap<&str, String> = HashMap::new();
    /// let client = ClientConfig::builder().uri("localhost:6334")
    ///     .api_key(config.get("api_key"))
    ///     .build();
    /// ```
    pub fn api_key(&mut self, key: impl MaybeApiKey) -> &mut Self {
        self.api_key = Some(key.maybe_key());
        self
    }
}
