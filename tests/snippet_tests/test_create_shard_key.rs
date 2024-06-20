
#[tokio::test]
async fn test_create_shard_key() {
    async fn create_shard_key() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_shard_key.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::{client::QdrantClient, qdrant::shard_key::Key};
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .create_shard_key(
                "{collection_name}",
                &Key::Keyword("{shard_key".to_string()),
                None,
                None,
                &[],
            )
            .await?;
        Ok(())
    }
    let _ = create_shard_key().await;
}
