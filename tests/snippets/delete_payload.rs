use qdrant_client::qdrant::{DeletePayloadPointsBuilder, PointsIdsList};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .delete_payload(
        DeletePayloadPointsBuilder::new(
            "{collection_name}",
            vec!["color".to_string(), "price".to_string()],
        )
        .points_selector(PointsIdsList {
            ids: vec![0.into(), 3.into(), 100.into()],
        })
        .wait(true),
    )
    .await?;
