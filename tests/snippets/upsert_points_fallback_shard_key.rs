use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder, ShardKeySelectorBuilder};
use qdrant_client::{Payload, Qdrant};
use serde_json::json;

let client = Qdrant::from_url("http://localhost:6334").build()?;

let points = vec![
    PointStruct::new(
        1,
        vec![0.05, 0.61, 0.76, 0.74],
        Payload::try_from(json!({
            "city": "Berlin", 
            "price": 1.99,
            "version": 3
        })).unwrap(),
    )
];

let shard_key_selector = ShardKeySelectorBuilder::with_shard_key("tenant-123")
    .fallback("default")
    .add_shard_key("tenant-123")
    .build();

client
    .upsert_points(
        UpsertPointsBuilder::new("{collection_name}", points)
            .wait(true)
            .shard_key_selector(shard_key_selector)
    ).await?;
