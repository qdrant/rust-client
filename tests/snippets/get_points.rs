use qdrant_client::qdrant::GetPointsBuilder;
use qdrant_client::qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .get_points(GetPointsBuilder::new(
        "{collection_name}",
        vec![0.into(), 30.into(), 100.into()],
    ))
    .await?;
