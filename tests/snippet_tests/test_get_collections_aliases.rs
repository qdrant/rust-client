
#[tokio::test]
async fn test_get_collections_aliases() {
    async fn get_collections_aliases() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/get_collections_aliases.rs` file
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client.list_aliases().await?;
        Ok(())
    }
    let _ = get_collections_aliases().await;
}
