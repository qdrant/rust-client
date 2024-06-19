
#[tokio::test]
async fn test_create_field_index() {
    async fn create_field_index() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_field_index.rs` file
        use qdrant_client::{client::QdrantClient, qdrant::FieldType};
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
        .create_field_index(
            "{collection_name}",
            "{field_name}",
            FieldType::Keyword,
            None,
            None,
        )
        .await?;
        Ok(())
    }
    let _ = create_field_index().await;
}
