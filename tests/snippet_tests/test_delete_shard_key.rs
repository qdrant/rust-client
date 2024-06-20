
#[tokio::test]
async fn test_delete_shard_key() {
    async fn delete_shard_key() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_shard_key.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::{client::QdrantClient, qdrant::shard_key::Key};
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .delete_shard_key("{collection_name}", &Key::Keyword("{shard_key".to_string()))
            .await?;
        Ok(())
    }
    let _ = delete_shard_key().await;
}
