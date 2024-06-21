
#[tokio::test]
async fn test_delete_shard_key() {
    async fn delete_shard_key() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_shard_key.rs` file
        use qdrant_client::qdrant::shard_key::Key;
        use qdrant_client::qdrant::DeleteShardKeyRequestBuilder;
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .delete_shard_key(
                DeleteShardKeyRequestBuilder::new("{collection_name}")
                    .key(Key::Keyword("{shard_key".to_string())),
            )
            .await?;
        Ok(())
    }
    let _ = delete_shard_key().await;
}
