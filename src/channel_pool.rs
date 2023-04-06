use std::future::Future;
use std::sync::RwLock;
use std::time::Duration;
use tonic::transport::{Channel, ClientTlsConfig, Uri};
use tonic::{Code, Status};
use tower::discover::Change;

pub struct ChannelPool {
    channel: RwLock<Option<Channel>>,
    uris: Vec<Uri>,
    grpc_timeout: Duration,
    connection_timeout: Duration,
}

impl ChannelPool {
    pub fn new(
        uri: Uri,
        grpc_timeout: Duration,
        connection_timeout: Duration,
    ) -> Self {
        Self::balance(vec![uri], grpc_timeout, connection_timeout)
    }

    pub fn balance(
        uris: Vec<Uri>,
        grpc_timeout: Duration,
        connection_timeout: Duration,
    ) -> Self {
        Self {
            channel: RwLock::new(None),
            uris,
            grpc_timeout,
            connection_timeout,
        }
    }

    async fn make_channel(&self) -> Result<Channel, Status> {
        let tls = match self.uris.first().unwrap().scheme_str() {
            None => false,
            Some(schema) => match schema {
                "http" => false,
                "https" => true,
                _ => {
                    return Err(Status::invalid_argument(format!(
                        "Unsupported schema: {}",
                        schema
                    )))
                }
            },
        };

        let (channel, service_discovery) = Channel::balance_channel(16);

        for (index, uri) in self.uris.iter().enumerate() {
            let mut endpoint = Channel::builder(uri.clone())
                .timeout(self.grpc_timeout)
                .connect_timeout(self.connection_timeout);

            if tls {
                endpoint = endpoint
                    .tls_config(ClientTlsConfig::new())
                    .map_err(|e| Status::internal(format!("Failed to create TLS config: {}", e)))?
            }

            service_discovery.send(Change::Insert(index, endpoint)).await.unwrap();
        }

        *self.channel.write().unwrap() = Some(channel.clone());

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
    pub async fn with_channel<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(Channel) -> O,
        allow_retry: bool,
    ) -> Result<T, Status> {
        let channel = self.get_channel().await?;

        let result: Result<T, Status> = f(channel).await;

        // Reconnect on failure to handle the case with domain name change.
        match result {
            Ok(res) => Ok(res),
            Err(err) => match err.code() {
                Code::Internal | Code::Unavailable | Code::Cancelled | Code::Unknown => {
                    self.drop_channel().await;
                    if allow_retry {
                        let channel = self.get_channel().await?;
                        Ok(f(channel).await?)
                    } else {
                        Err(err)
                    }
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
        .await
        .expect("get channel should not error");
    });
}
