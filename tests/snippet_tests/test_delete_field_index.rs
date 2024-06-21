
#[tokio::test]
async fn test_delete_field_index() {
    async fn delete_field_index() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_field_index.rs` file
        use qdrant_client::qdrant::DeleteFieldIndexCollectionBuilder;
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .delete_field_index(DeleteFieldIndexCollectionBuilder::new(
                "{collection_name}",
                "{field_name}",
            ))
            .await?;
        Ok(())
    }
    let _ = delete_field_index().await;
}
