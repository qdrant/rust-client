use qdrant_client::qdrant::{ClearPayloadPointsBuilder, PointsIdsList};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .clear_payload(ClearPayloadPointsBuilder::new("{collection_name}").points(
        PointsIdsList {
            ids: vec![0.into(), 3.into(), 100.into()],
        },
    ))
    .await?;
