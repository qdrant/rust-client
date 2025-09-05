use std::future::Future;
use std::time::Duration;

use parking_lot::{Mutex, RwLock};
use tonic::transport::{Channel, ClientTlsConfig, Uri};
use tonic::{Code, Status};

pub struct ChannelPool {
    channels: RwLock<Vec<Option<Channel>>>,
    channel_index_lock: Mutex<usize>,
    uri: Uri,
    grpc_timeout: Duration,
    connection_timeout: Duration,
    keep_alive_while_idle: bool,
    pool_size: usize,
}

impl ChannelPool {
    pub fn new(
        uri: Uri,
        grpc_timeout: Duration,
        connection_timeout: Duration,
        keep_alive_while_idle: bool,
        mut pool_size: usize,
    ) -> Self {
        // Ensure `pool_size` is always >= 1
        pool_size = std::cmp::max(pool_size, 1);

        Self {
            channels: RwLock::new(vec![None; pool_size]),
            channel_index_lock: Mutex::new(0),
            uri,
            grpc_timeout,
            connection_timeout,
            keep_alive_while_idle,
            pool_size,
        }
    }

    /// Creates a new channel at the given index. If one already exists, it will be dropped and replaced.
    async fn make_channel(&self, channel_index: usize) -> Result<Channel, Status> {
        let tls = match self.uri.scheme_str() {
            None => false,
            Some(schema) => match schema {
                "http" => false,
                "https" => true,
                _ => {
                    return Err(Status::invalid_argument(format!(
                        "Unsupported schema: {schema}"
                    )))
                }
            },
        };

        let rust_client_version = env!("CARGO_PKG_VERSION").to_string();
        let version_info = format!("rust-client/{rust_client_version}");

        let endpoint = Channel::builder(self.uri.clone())
            .timeout(self.grpc_timeout)
            .connect_timeout(self.connection_timeout)
            .keep_alive_while_idle(self.keep_alive_while_idle)
            .user_agent(version_info)
            .expect("Version info should be a valid header value");

        let endpoint = if tls {
            let tls_config = ClientTlsConfig::new().with_native_roots();
            endpoint
                .tls_config(tls_config)
                .map_err(|e| Status::internal(format!("Failed to create TLS config: {e}")))?
        } else {
            endpoint
        };

        let new_channel = endpoint
            .connect()
            .await
            .map_err(|e| Status::internal(format!("Failed to connect to {}: {:?}", self.uri, e)))?;

        let mut pool_channels = self.channels.write();
        pool_channels[channel_index] = Some(new_channel.clone());
        Ok(new_channel)
    }

    /// Returns a channel from the pool. If `pool_size` > 1, calls will return different channels in a round-robin way.
    /// Otherwise, the same channel is returned each time.
    async fn get_channel(&self) -> Result<(Channel, usize), Status> {
        let channel_index = self.next_channel_index();

        if let Some(channel) = self
            .channels
            .read()
            .get(channel_index)
            .and_then(|i| i.as_ref())
        {
            return Ok((channel.clone(), channel_index));
        }

        Ok((self.make_channel(channel_index).await?, channel_index))
    }

    /// Drops the channel at the given index.
    fn drop_channel(&self, idx: usize) {
        let mut channel = self.channels.write();
        channel[idx] = None;
    }

    // Allow to retry request if channel is broken
    pub async fn with_channel<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(Channel) -> O,
        allow_retry: bool,
    ) -> Result<T, Status> {
        let (channel, channel_index) = self.get_channel().await?;

        let result: Result<T, Status> = f(channel).await;

        // Reconnect on failure to handle the case with domain name change.
        match result {
            Ok(res) => Ok(res),
            Err(err) => match err.code() {
                Code::Internal | Code::Unavailable | Code::Cancelled | Code::Unknown => {
                    if allow_retry {
                        // Recreate the channel at the same index when reconnecting.
                        let channel = self.make_channel(channel_index).await?;
                        Ok(f(channel).await?)
                    } else {
                        // If retries aren't allowed, delete the channel so it will be recreated
                        // the next time it's used.
                        self.drop_channel(channel_index);
                        Err(err)
                    }
                }
                _ => Err(err)?,
            },
        }
    }

    /// Returns `true` if multiple connections being used.
    fn is_connection_pooling_enabled(&self) -> bool {
        // This value is never `0` becuase we enforce this in the constructor.
        // 1 connection = No pooling
        self.pool_size != 1
    }

    /// Returns the index for the next channel to use.
    fn next_channel_index(&self) -> usize {
        // Avoid the expensive locking operation if pooling is disabled.
        if !self.is_connection_pooling_enabled() {
            return 0;
        }

        // ChannelIndex always holds the index of the next client to return.
        // Therefore we increase the counter and return the current index.
        let mut channel_index = self.channel_index_lock.lock();
        let curr_idx = *channel_index;
        let next = (curr_idx + 1) % self.pool_size;
        *channel_index = next;
        curr_idx
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
            2,
        )
        .get_channel()
        .await
        .expect("get channel should not error");
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_channel_counter() {
        let channel = ChannelPool::new(
            Uri::from_static("http://localhost:6444"),
            Duration::default(),
            Duration::default(),
            false,
            5,
        );

        assert_eq!(channel.next_channel_index(), 0);
        assert_eq!(channel.next_channel_index(), 1);
        assert_eq!(channel.next_channel_index(), 2);
        assert_eq!(channel.next_channel_index(), 3);
        assert_eq!(channel.next_channel_index(), 4);
        assert_eq!(channel.next_channel_index(), 0);
        assert_eq!(channel.next_channel_index(), 1);

        assert_eq!(channel.channels.read().len(), 5);
    }
}
