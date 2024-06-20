
#[tokio::test]
async fn test_list_full_snapshots() {
    async fn list_full_snapshots() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/list_full_snapshots.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.list_full_snapshots().await?;
        Ok(())
    }
    let _ = list_full_snapshots().await;
}
