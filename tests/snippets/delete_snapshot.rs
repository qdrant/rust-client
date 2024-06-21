use qdrant_client::qdrant::DeleteSnapshotRequestBuilder;
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .delete_snapshot(DeleteSnapshotRequestBuilder::new(
        "{collection_name}",
        "{snapshot_name}",
    ))
    .await?;
