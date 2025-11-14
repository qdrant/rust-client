
#[tokio::test]
async fn test_replicate_points() {
    async fn replicate_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/replicate_points.rs` file
        use qdrant_client::qdrant::{
            update_collection_cluster_setup_request::Operation, Condition, Filter,
            ReplicatePointsBuilder, ShardKey, UpdateCollectionClusterSetupRequest,
        };
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        // Replicate all points from one shard to another
        client
            .update_collection_cluster_setup(UpdateCollectionClusterSetupRequest {
                collection_name: "{collection_name}".to_string(),
                operation: Some(Operation::ReplicatePoints(
                    ReplicatePointsBuilder::new(
                        ShardKey::from("source_shard".to_string()),
                        ShardKey::from("target_shard".to_string()),
                    )
                    .into(),
                )),
                timeout: None,
            })
            .await?;
        
        // Replicate only specific points matching a filter
        client
            .update_collection_cluster_setup(UpdateCollectionClusterSetupRequest {
                collection_name: "{collection_name}".to_string(),
                operation: Some(Operation::ReplicatePoints(
                    ReplicatePointsBuilder::new(
                        ShardKey::from("source_shard".to_string()),
                        ShardKey::from("target_shard".to_string()),
                    )
                    .filter(Filter::must([Condition::matches(
                        "status",
                        "active".to_string(),
                    )]))
                    .into(),
                )),
                timeout: None,
            })
            .await?;
        Ok(())
    }
    let _ = replicate_points().await;
}
