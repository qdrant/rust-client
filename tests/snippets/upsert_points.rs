// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::{client::QdrantClient, client::Payload, qdrant::PointStruct};
use serde_json::json;

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .upsert_points_batch_blocking(
        "{collection_name}".to_string(),
        None,
        vec![
            PointStruct::new(
                1,
                vec![0.9, 0.1, 0.1],
                Payload::try_from(json!(
                    {"color": "red"}
                )).unwrap(),
            ),
            PointStruct::new(
                2,
                vec![0.1, 0.9, 0.1],
                Payload::try_from(json!(
                    {"color": "green"}
                )).unwrap(),
            ),
            PointStruct::new(
                3,
                vec![0.1, 0.1, 0.9],
                Payload::try_from(json!(
                    {"color": "blue"}
                )).unwrap(),
            ),
        ],
        None,
        100,
    )
    .await?;
