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
    // Example of top level client
    // You may also use tonic-generated client from `src/qdrant.rs`
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config)).await?;

    let collections_list = client.list_collections().await?;
    // ListCollectionsResponse { collections: [CollectionDescription { name: "test" }], time: 3.27e-6 }

    let collection_name = "test";
    client.delete_collection(collection_name).await?;

    client
        .create_collection(CreateCollection {
            collection_name: collection_name.into(),
            vector_size: 10,
            distance: Distance::Cosine.into(),
            ..Default::default()
        })
        .await?;

    let collection_info = client.collection_info(collection_name).await?;

    let payload: Payload = vec![
        ("foo", "Bar".into()),
        ("bar", 12.into()),
    ].into_iter().collect::<HashMap<_, Value>>().into();

    let points = vec![
        PointStruct::new(0, vec![12.; 10], payload)
    ];
    client.upsert_points_blocking(collection_name, points).await?;

    let search_result = client
        .search_points(SearchPoints {
            collection_name: collection_name.into(),
            vector: vec![11.; 10],
            filter: None,
            limit: 10,
            with_vector: None,
            with_payload: None,
            params: None,
            score_threshold: None,
            offset: None,
        })
        .await?;
    // search_result = SearchResponse {
    //     result: [
    //         ScoredPoint {
    //         id: Some(
    //             PointId {
    //                 point_id_options: Some(Num(0)),
    //             },
    //         ),
    //         payload: {},
    //         score: 1.0000001,
    //         vector: [],
    //         version: 0,
    //     },
    //     ],
    //     time: 5.312e-5,
    // }

}
```
