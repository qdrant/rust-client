use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder, Filter, Condition};
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

client
    .upsert_points(
        UpsertPointsBuilder::new("{collection_name}", points)
            .wait(true)
            .update_filter(Filter::must([Condition::matches("version", 2)]))
    ).await?;