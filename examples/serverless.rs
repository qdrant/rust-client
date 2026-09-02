//! Example of using the Qdrant Serverless client.
//!
//! Collection management uses the simplified serverless API; point operations
//! (query, upsert, ...) work like in the regular client (unsupported cluster
//! parameters are stripped before each RPC).
//!
//! ```bash
//! cargo run --example serverless --features serde
//! ```

use qdrant_client::qdrant::{PointStruct, QueryPointsBuilder, UpsertPointsBuilder};
use qdrant_client::serverless::{
    CollectionConfig, DenseVectorConfig, Distance, KeywordIndex, QdrantServerless,
};
use qdrant_client::{Payload, QdrantError};

#[tokio::main]
async fn main() -> Result<(), QdrantError> {
    let client = QdrantServerless::from_url(
        "https://serverless.plush-volt.aws.development-cloud.qdrant.io",
    )
    .api_key("<your api key>")
    .build()?;

    let collection_name = "my-collection";

    // make the example rerunnable: creating an existing collection returns ALREADY_EXISTS
    if client.collection_exists(collection_name).await? {
        client.delete_collection(collection_name).await?;
    }

    // serverless-specific collection management: no quantization, wal,
    // segment number etc. - the serverless manager decides those
    let result = client
        .create_collection(
            collection_name,
            CollectionConfig::new()
                .dense_vector(DenseVectorConfig::new(4, Distance::Cosine))
                .payload_index("color", KeywordIndex),
        )
        .await?;
    println!("create_collection: {result}");

    println!("collections: {:?}", client.list_collections().await?);
    println!("info: {:?}", client.get_collection(collection_name).await?);

    let points = vec![
        PointStruct::new(
            1,
            vec![0.1, 0.2, 0.3, 0.4],
            Payload::try_from(serde_json::json!({"color": "red"})).unwrap(),
        ),
        PointStruct::new(
            2,
            vec![0.4, 0.3, 0.2, 0.1],
            Payload::try_from(serde_json::json!({"color": "blue"})).unwrap(),
        ),
    ];
    client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points))
        .await?;

    let hits = client
        .query(
            QueryPointsBuilder::new(collection_name)
                .query(vec![0.1, 0.2, 0.3, 0.4])
                .limit(10),
        )
        .await?;
    println!("query: {hits:?}");

    client.delete_collection(collection_name).await?;
    Ok(())
}
