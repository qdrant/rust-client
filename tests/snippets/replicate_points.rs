use qdrant_client::qdrant::{
    update_collection_cluster_setup_request::Operation, Condition, Filter,
    ReplicatePointsBuilder, ShardKey, UpdateCollectionClusterSetupRequest,
};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .update_collection_cluster_setup(UpdateCollectionClusterSetupRequest {
        collection_name: "{collection_name}".to_string(),
        operation: Some(Operation::ReplicatePoints(
            ReplicatePointsBuilder::new(
                ShardKey::from("source_shard"),
                ShardKey::from("target_shard"),
            )
            .filter(Filter::must([Condition::matches(
                "user",
                "tenant-123".to_string(),
            )]))
            .build(),
        )),
        timeout: None,
    })
    .await?;
