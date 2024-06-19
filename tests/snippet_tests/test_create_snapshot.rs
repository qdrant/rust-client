
#[tokio::test]
async fn test_create_snapshot() {
    async fn create_snapshot() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_snapshot.rs` file
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.create_snapshot("{collection_name}").await?;
        Ok(())
    }
    let _ = create_snapshot().await;
}
