use qdrant_client::{client::QdrantClient, qdrant::OptimizersConfigDiff};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .update_collection(
        "{collection_name}",
        Some(&OptimizersConfigDiff {
            indexing_threshold: Some(10000),
            ..Default::default()
        }),
        None,
        None,
        None,
        None,
        None,
    )
    .await?;
