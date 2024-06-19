
#[tokio::test]
async fn test_delete_collection() {
    async fn delete_collection() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_collection.rs` file
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.delete_collection("{collection_name}").await?;
        Ok(())
    }
    let _ = delete_collection().await;
}
