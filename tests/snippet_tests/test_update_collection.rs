
#[tokio::test]
async fn test_update_collection() {
    async fn update_collection() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/update_collection.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::{client::QdrantClient, qdrant::OptimizersConfigDiff};
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .update_collection(
                "{collection_name}",
                Some(&OptimizersConfigDiff {
                    indexing_threshold: Some(10000),
                    ..Default::default()
                }),
                None,
                None,
                None,
                None,
                None,
            )
            .await?;
        Ok(())
    }
    let _ = update_collection().await;
}
