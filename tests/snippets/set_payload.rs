// TODO: remove this once this test has been converted
#![allow(deprecated)]

use serde_json::json;
use qdrant_client::{client::QdrantClient, qdrant::{
    points_selector::PointsSelectorOneOf, PointsIdsList, PointsSelector,
}};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .set_payload_blocking(
        "{collection_name}",
        None,
        &PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                ids: vec![0.into(), 3.into(), 10.into()],
            })),
        },
        json!({
            "property1": "string",
            "property2": "string",
        })
        .try_into()
        .unwrap(),
        None,
        None,
    )
    .await?;
