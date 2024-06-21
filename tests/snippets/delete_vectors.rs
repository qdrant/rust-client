use qdrant_client::qdrant::{DeletePointVectorsBuilder, PointsIdsList, VectorsSelector};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .delete_vectors(
        DeletePointVectorsBuilder::new("{collection_name}")
            .points_selector(PointsIdsList {
                ids: vec![0.into(), 3.into(), 10.into()],
            })
            .vectors(VectorsSelector {
                names: vec!["text".into(), "image".into()],
            })
            .wait(true),
    )
    .await?;
