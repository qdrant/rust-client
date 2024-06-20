
#[tokio::test]
async fn test_delete_full_snapshot() {
    async fn delete_full_snapshot() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_full_snapshot.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.delete_full_snapshot("{snapshot_name}").await?;
        Ok(())
    }
    let _ = delete_full_snapshot().await;
}
