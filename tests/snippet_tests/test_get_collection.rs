
#[tokio::test]
async fn test_get_collection() {
    async fn get_collection() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/get_collection.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.collection_info("{collection_name}").await?;
        Ok(())
    }
    let _ = get_collection().await;
}
