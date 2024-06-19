
#[tokio::test]
async fn test_collection_exists() {
    async fn collection_exists() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/collection_exists.rs` file
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.collection_exists("{collection_name}").await?;
        Ok(())
    }
    let _ = collection_exists().await;
}
