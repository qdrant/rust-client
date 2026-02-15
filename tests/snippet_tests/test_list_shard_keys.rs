
#[tokio::test]
async fn test_list_shard_keys() {
    async fn list_shard_keys() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/list_shard_keys.rs` file
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let _response = client.list_shard_keys("{collection_name}").await?;
        Ok(())
    }
    let _ = list_shard_keys().await;
}
