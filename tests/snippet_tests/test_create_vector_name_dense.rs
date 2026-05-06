
#[tokio::test]
async fn test_create_vector_name_dense() {
    async fn create_vector_name_dense() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateVectorNameRequestBuilder, DenseVectorCreationConfigBuilder, Distance,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .create_vector_name(
                CreateVectorNameRequestBuilder::new(
                    "{collection_name}",
                    "image",
                    DenseVectorCreationConfigBuilder::new(512, Distance::Cosine),
                )
                .wait(true),
            )
            .await?;
        Ok(())
    }
    let _ = create_vector_name_dense().await;
}
