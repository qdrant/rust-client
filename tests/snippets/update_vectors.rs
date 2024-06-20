// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::client::QdrantClient;
use qdrant_client::qdrant::PointVectors;
use std::collections::HashMap;

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .update_vectors_blocking(
        "{collection_name}",
        None,
        &[
            PointVectors {
                id: Some(1.into()),
                vectors: Some(
                    HashMap::from([("image".to_string(), vec![0.1, 0.2, 0.3, 0.4])]).into(),
                ),
            },
            PointVectors {
                id: Some(2.into()),
                vectors: Some(
                    HashMap::from([(
                        "text".to_string(),
                        vec![0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2],
                    )])
                    .into(),
                ),
            },
        ],
        None,
    )
    .await?;
