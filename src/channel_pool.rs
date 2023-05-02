use std::future::Future;
use std::time::Duration;
use tonic::transport::{Channel, ClientTlsConfig, Uri};
use tonic::Status;
#[derive(Clone)]
pub struct ChannelPool {
    channel: Channel,
}
async fn make_channel(
    uri: Uri,
    grpc_timeout: Duration,
    connection_timeout: Duration,
    keep_alive_while_idle: bool,
) -> Result<Channel, Status> {
    let tls = match uri.scheme_str() {
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

    let endpoint = Channel::builder(uri.clone())
        .timeout(grpc_timeout)
        .connect_timeout(connection_timeout)
        .keep_alive_while_idle(keep_alive_while_idle);

    let endpoint = if tls {
        endpoint
            .tls_config(ClientTlsConfig::new())
            .map_err(|e| Status::internal(format!("Failed to create TLS config: {}", e)))?
    } else {
        endpoint
    };

    let channel = endpoint
        .connect()
        .await
        .map_err(|e| Status::internal(format!("Failed to connect to {}: {}", uri, e)))?;

    Ok(channel)
}
impl ChannelPool {
    pub async fn new(
        uri: Uri,
        grpc_timeout: Duration,
        connection_timeout: Duration,
        keep_alive_while_idle: bool,
    ) -> Result<Self, Status> {
        let channel = make_channel(
            uri.clone(),
            grpc_timeout,
            connection_timeout,
            keep_alive_while_idle,
        )
        .await?;
        Ok(Self { channel })
    }

    // Allow to retry request if channel is broken
    pub async fn with_channel<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(Channel) -> O,
    ) -> Result<T, Status> {
        f(self.channel.clone()).await
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
        .await
        .expect("get channel should not error");
    });
}
