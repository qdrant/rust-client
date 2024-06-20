
#[tokio::test]
async fn test_list_snapshots() {
    async fn list_snapshots() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/list_snapshots.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.list_snapshots("{collection_name}").await?;
        Ok(())
    }
    let _ = list_snapshots().await;
}
