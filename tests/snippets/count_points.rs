use qdrant_client::{client::QdrantClient, qdrant::{Condition, CountPoints, Filter}};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .count(&CountPoints {
        collection_name: "{collection_name}".to_string(),
        filter: Some(Filter::must([Condition::matches(
            "color",
            "red".to_string(),
        )])),
        exact: Some(true),
        ..Default::default()
    })
    .await?;
