// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::{
    client::QdrantClient,
    qdrant::{Condition, Filter, RecommendPoints, RecommendStrategy},
};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .recommend(&RecommendPoints {
        collection_name: "{collection_name}".to_string(),
        positive: vec![100.into(), 200.into()],
        positive_vectors: vec![vec![100.0, 231.0].into()],
        negative: vec![718.into()],
        negative_vectors: vec![vec![0.2, 0.3, 0.4, 0.5].into()],
        strategy: Some(RecommendStrategy::AverageVector.into()),
        filter: Some(Filter::must([Condition::matches(
            "city",
            "London".to_string(),
        )])),
        limit: 3,
        ..Default::default()
    })
    .await?;
