
#[tokio::test]
async fn test_create_shard_key() {
    async fn create_shard_key() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_shard_key.rs` file
        use qdrant_client::qdrant::shard_key::Key;
        use qdrant_client::qdrant::{CreateShardKeyBuilder, CreateShardKeyRequestBuilder};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .create_shard_key(
                CreateShardKeyRequestBuilder::new("{collection_name}").request(
                    CreateShardKeyBuilder::default()
                        .shard_key(Key::Keyword("{shard_key}".to_string())),
                ),
            )
            .await?;
        Ok(())
    }
    let _ = create_shard_key().await;
}
