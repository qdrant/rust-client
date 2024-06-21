
#[tokio::test]
async fn test_update_collection() {
    async fn update_collection() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/update_collection.rs` file
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
        Ok(())
    }
    let _ = update_collection().await;
}
