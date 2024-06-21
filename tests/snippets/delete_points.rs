use qdrant_client::qdrant::{Condition, DeletePointsBuilder, Filter, PointsIdsList};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .delete_points(
        DeletePointsBuilder::new("{collection_name}")
            .points(PointsIdsList {
                ids: vec![0.into(), 3.into(), 100.into()],
            })
            .wait(true),
    )
    .await?;

client
    .delete_points(
        DeletePointsBuilder::new("{collection_name}")
            .points(Filter::must([Condition::matches(
                "color",
                "red".to_string(),
            )]))
            .wait(true),
    )
    .await?;
