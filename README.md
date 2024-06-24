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

## Dependencies

The client uses gRPC via the [Tonic](https://github.com/hyperium/tonic) library.

To change anything in the protocol buffer definitions, you need the `protoc` Protocol Buffers compiler, along with Protocol Buffers resource files.

Refer to the [Tonic installation guide](https://github.com/hyperium/tonic#dependencies) for more details.

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

Add necessary dependencies:

```bash
cargo add qdrant-client anyhow tonic tokio serde-json --features tokio/rt-multi-thread
```

Add search example from [`examples/search.rs`](./examples/search.rs) to your `src/main.rs`:

```rust
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, ScalarQuantizationBuilder,
    SearchParamsBuilder, SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant, QdrantError};

#[tokio::main]
async fn main() -> Result<(), QdrantError> {
    // Example of top level client
    // You may also use tonic-generated client from `src/qdrant.rs`
    let client = Qdrant::from_url("http://localhost:6334").build()?;

    let collections_list = client.list_collections().await?;
    dbg!(collections_list);
    // collections_list = {
    //   "collections": [
    //     {
    //       "name": "test"
    //     }
    //   ]
    // }

    let collection_name = "test";
    client.delete_collection(collection_name).await?;

    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(10, Distance::Cosine))
                .quantization_config(ScalarQuantizationBuilder::default()),
        )
        .await?;

    let collection_info = client.collection_info(collection_name).await?;
    dbg!(collection_info);

    let payload: Payload = serde_json::json!(
        {
            "foo": "Bar",
            "bar": 12,
            "baz": {
                "qux": "quux"
            }
        }
    )
    .try_into()
    .unwrap();

    let points = vec![PointStruct::new(0, vec![12.; 10], payload)];
    client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points))
        .await?;

    let search_result = client
        .search_points(
            SearchPointsBuilder::new(collection_name, [11.; 10], 10)
                .filter(Filter::all([Condition::matches("bar", 12)]))
                .with_payload(true)
                .params(SearchParamsBuilder::default().exact(true)),
        )
        .await?;
    dbg!(&search_result);
    // search_result = [
    //   {
    //     "id": 0,
    //     "version": 0,
    //     "score": 1.0000001,
    //     "payload": {
    //       "bar": 12,
    //       "baz": {
    //         "qux": "quux"
    //       },
    //       "foo": "Bar"
    //     }
    //   }
    // ]

    let found_point = search_result.result.into_iter().next().unwrap();
    let mut payload = found_point.payload;
    let baz_payload = payload.remove("baz").unwrap().into_json();
    println!("baz: {}", baz_payload);
    // baz: {"qux":"quux"}

    Ok(())
}
```

Or run the example from this project directly:

```bash
cargo run --example search
```

## Qdrant Cloud

[Qdrant Cloud](https://cloud.qdrant.io) is a managed service for Qdrant.

The client needs to be configured properly to access the service.

- make sure to use the correct port (6334)
- make sure to pass your API KEY

```rust
async fn make_client() -> Result<QdrantClient> {
    let client = QdrantClient::from_url("http://xxxxxxxxxx.eu-central.aws.cloud.qdrant.io:6334")
        // using an env variable for the API KEY for example
        .with_api_key(std::env::var("QDRANT_API_KEY"))
        .build()?;
    Ok(client)
}
```
