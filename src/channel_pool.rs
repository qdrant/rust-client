use std::future::Future;
use std::sync::RwLock;
use std::time::Duration;
use tonic::{Code, Status};
use tonic::transport::{Channel, ClientTlsConfig, Uri};

pub struct ChannelPool {
    channel: RwLock<Option<Channel>>,
    uri: Uri,
    grpc_timeout: Duration,
    connection_timeout: Duration,
    keep_alive_while_idle: bool,
}

impl ChannelPool {
    pub fn new(uri: Uri, grpc_timeout: Duration, connection_timeout: Duration, keep_alive_while_idle: bool) -> Self {
        Self {
            channel: RwLock::new(None),
            uri,
            grpc_timeout,
            connection_timeout,
            keep_alive_while_idle,
        }
    }

    async fn make_channel(&self) -> Result<Channel, Status> {
        let tls = match self.uri.scheme_str() {
            None => false,
            Some(schema) => match schema {
                "http" => false,
                "https" => true,
                _ => return Err(Status::invalid_argument(format!("Unsupported schema: {}", schema))),
            }
        };

        let endpoint = Channel::builder(self.uri.clone())
            .timeout(self.grpc_timeout)
            .connect_timeout(self.connection_timeout)
            .keep_alive_while_idle(self.keep_alive_while_idle);

        let endpoint = if tls {
            endpoint.tls_config(ClientTlsConfig::new()).map_err(|e| Status::internal(format!("Failed to create TLS config: {}", e)))?
        } else {
            endpoint
        };


        let channel = endpoint.connect().await.map_err(|e| Status::internal(format!("Failed to connect to {}: {}", self.uri, e)))?;
        let mut self_channel = self.channel.write().unwrap();

        *self_channel = Some(channel.clone());

        Ok(channel)
    }

    async fn get_channel(&self) -> Result<Channel, Status> {
        if let Some(channel) = &*self.channel.read().unwrap() {
            return Ok(channel.clone());
        }
        
        let channel = self.make_channel().await?;
        Ok(channel)
    }

    pub async fn drop_channel(&self) {
        let mut channel = self.channel.write().unwrap();
        *channel = None;
    }

    // Allow to retry request if channel is broken
    pub async fn with_channel<T, O: Future<Output=Result<T, Status>>>(
        &self,
        f: impl Fn(Channel) -> O,
    ) -> Result<T, Status> {
        let channel = self.get_channel().await?;

        let result: Result<T, Status> = f(channel).await;

        // Reconnect on failure to handle the case with domain name change.
        match result {
            Ok(res) => Ok(res),
            Err(err) => match err.code() {
                Code::Internal | Code::Unavailable | Code::Cancelled | Code::Unknown => {
                    self.drop_channel().await;
                    let channel = self.get_channel().await?;
                    Ok(f(channel).await?)
                }
                _ => Err(err)?,
            },
        }
    }
}

// The future returned by get_channel needs to be Send so that the client can be 
// used by external async functions.
#[test]
fn require_get_channel_fn_to_be_send() {
    fn require_send<T: Send>(_t: T) {}
    require_send(async {
        ChannelPool::new(
            Uri::from_static(""),
            Duration::from_millis(0),
            Duration::from_millis(0),
            false,
        )
        .get_channel()
        .await.expect("get channel should not error");
    });
}
