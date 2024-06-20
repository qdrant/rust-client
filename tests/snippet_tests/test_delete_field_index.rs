
#[tokio::test]
async fn test_delete_field_index() {
    async fn delete_field_index() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_field_index.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .delete_field_index("{collection_name}", "{field_name}", None)
            .await?;
        Ok(())
    }
    let _ = delete_field_index().await;
}
