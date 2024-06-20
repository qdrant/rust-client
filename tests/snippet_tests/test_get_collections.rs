
#[tokio::test]
async fn test_get_collections() {
    async fn get_collections() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/get_collections.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.list_collections().await?;
        Ok(())
    }
    let _ = get_collections().await;
}
