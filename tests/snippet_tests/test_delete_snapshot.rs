
#[tokio::test]
async fn test_delete_snapshot() {
    async fn delete_snapshot() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_snapshot.rs` file
        use qdrant_client::qdrant::DeleteSnapshotRequestBuilder;
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .delete_snapshot(DeleteSnapshotRequestBuilder::new(
                "{collection_name}",
                "{snapshot_name}",
            ))
            .await?;
        Ok(())
    }
    let _ = delete_snapshot().await;
}
