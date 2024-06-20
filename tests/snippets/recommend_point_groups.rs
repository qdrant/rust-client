// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::{
    client::QdrantClient,
    qdrant::{RecommendPointGroups, RecommendStrategy},
};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .recommend_groups(&RecommendPointGroups {
        collection_name: "{collection_name}".to_string(),
        group_by: "document_id".to_string(),
        group_size: 2,
        positive: vec![100.into(), 200.into()],
        negative: vec![718.into()],
        strategy: Some(RecommendStrategy::AverageVector.into()),
        limit: 3,
        ..Default::default()
    })
    .await?;
