# rust-client

Rust client for Qdrant vector search engine.

[![Crates.io][crates-badge]][crates-url]
[![Apache 2.0 licensed][apache2-badge]][apache2-url]

[crates-badge]: https://img.shields.io/crates/v/qdrant-client.svg
[crates-url]: https://crates.io/crates/qdrant-client
[apache2-badge]: https://img.shields.io/badge/license-apache2-blue.svg
[apache2-url]: https://github.com/qdrant/rust-client/blob/master/LICENSE

## Installation

```bash
cargo add qdrant-client
```

Package is available in [crates.io](https://crates.io/crates/qdrant-client)

## Usage

Run Qdrant with enabled gRPC interface:

```bash
# With env variable
docker run -p 6333:6333 -p 6334:6334 \
    -e QDRANT__SERVICE__GRPC_PORT="6334" \
    qdrant/qdrant
```
Or by updating the configuration file:

```yaml
service:
  grpc_port: 6334
```

More info about gRPC in [documentation](https://qdrant.tech/documentation/quick_start/#grpc).

### Making requests

```rust
use std::time::Duration;
use tonic::transport::Channel;
use qdrant_client::QdrantClient;
use qdrant_client::qdrant::ListCollectionsRequest;

#[tokio::main]
async fn main() {
    println!("Hello, qdrant!");

    let uri = "http://localhost:6334".parse().unwrap();
    let endpoint = Channel::builder(uri)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(5))
        .keep_alive_while_idle(true);

    // `connect` is using the `Reconnect` network service internally to handle dropped connections
    let channel = endpoint.connect().await.unwrap(); // Unwrap in test only

    let mut client = QdrantClient::new(channel);
    let collections_list = client.collection_api.list(ListCollectionsRequest {}).await.unwrap();
    println!("Collection count {:?}", collections_list.into_inner().collections.len());
}
```
