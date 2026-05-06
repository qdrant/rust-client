
#[tokio::test]
async fn test_create_vector_name_sparse() {
    async fn create_vector_name_sparse() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateVectorNameRequestBuilder, Modifier, SparseVectorCreationConfigBuilder,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .create_vector_name(
                CreateVectorNameRequestBuilder::new(
                    "{collection_name}",
                    "text_sparse",
                    SparseVectorCreationConfigBuilder::new().modifier(Modifier::Idf),
                )
                .wait(true),
            )
            .await?;
        Ok(())
    }
    let _ = create_vector_name_sparse().await;
}
