use qdrant_client::qdrant::{OptimizersConfigDiffBuilder, UpdateCollectionBuilder};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .update_collection(
        UpdateCollectionBuilder::new("{collection_name}").optimizers_config(
            OptimizersConfigDiffBuilder::default().indexing_threshold(10_000),
        ),
    )
    .await?;
