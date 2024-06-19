
#[tokio::test]
async fn test_create_full_snapshot() {
    async fn create_full_snapshot() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_full_snapshot.rs` file
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.create_full_snapshot().await?;
        Ok(())
    }
    let _ = create_full_snapshot().await;
}
