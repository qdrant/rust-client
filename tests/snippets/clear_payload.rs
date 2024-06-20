// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::{client::QdrantClient, qdrant::{
    points_selector::PointsSelectorOneOf, PointsIdsList, PointsSelector,
}};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .clear_payload(
        "{collection_name}",
        None,
        Some(PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                ids: vec![0.into(), 3.into(), 100.into()],
            })),
        }),
        None,
    )
    .await?;
