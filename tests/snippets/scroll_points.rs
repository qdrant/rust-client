use qdrant_client::{client::QdrantClient, qdrant::{Condition, Filter, ScrollPoints}};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .scroll(&ScrollPoints {
        collection_name: "{collection_name}".to_string(),
        filter: Some(Filter::must([Condition::matches(
            "color",
            "red".to_string(),
        )])),
        limit: Some(1),
        with_payload: Some(true.into()),
        with_vectors: Some(false.into()),
        ..Default::default()
    })
    .await?;
