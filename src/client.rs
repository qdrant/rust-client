use crate::QdrantClient;
use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;
use tonic::transport::Channel;

pub struct ClientConfig {
    pub uri: String,
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub keep_alive_while_idle: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            uri: String::from("http://[::1]:6334"),
            timeout: Duration::from_secs(5),
            connect_timeout: Duration::from_secs(5),
            keep_alive_while_idle: true,
        }
    }
}

#[async_trait]
pub trait QdrantClientExtensions {
    async fn new(cfg: Option<ClientConfig>) -> Result<Self>
    where
        Self: Sized;
}

#[async_trait]
impl QdrantClientExtensions for QdrantClient {
    async fn new(cfg: Option<ClientConfig>) -> Result<Self> {
        let cfg = cfg.unwrap_or_default();

        let endpoint = Channel::builder(cfg.uri.parse().unwrap())
            .timeout(cfg.timeout)
            .connect_timeout(cfg.connect_timeout)
            .keep_alive_while_idle(cfg.keep_alive_while_idle);
        let channel = endpoint.connect().await?;
        let client = QdrantClient::new(channel);

        Ok(client)
    }
}
