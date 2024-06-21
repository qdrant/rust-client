
#[tokio::test]
async fn test_get_collection_aliases() {
    async fn get_collection_aliases() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/get_collection_aliases.rs` file
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.list_collection_aliases("{collection_name}").await?;
        Ok(())
    }
    let _ = get_collection_aliases().await;
}
