
#[tokio::test]
async fn test_create_collection() {
    async fn create_collection() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_collection.rs` file
        use qdrant_client::qdrant::{Distance, VectorParamsBuilder};
        use qdrant_client::builders::CreateCollectionBuilder;
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .create_collection(
                CreateCollectionBuilder::new("{collection_name}")
                    .vectors_config(VectorParamsBuilder::new(100, Distance::Cosine)),
            )
            .await?;
        Ok(())
    }
    let _ = create_collection().await;
}
