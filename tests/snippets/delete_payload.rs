use qdrant_client::{client::QdrantClient, qdrant::{
    points_selector::PointsSelectorOneOf, PointsIdsList, PointsSelector,
}};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .delete_payload_blocking(
        "{collection_name}",
        None,
        &PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                ids: vec![0.into(), 3.into(), 100.into()],
            })),
        },
        vec!["color".to_string(), "price".to_string()],
        None,
    )
    .await?;
