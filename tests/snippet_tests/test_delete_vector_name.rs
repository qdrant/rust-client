
#[tokio::test]
async fn test_delete_vector_name() {
    async fn delete_vector_name() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::DeleteVectorNameRequestBuilder;
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .delete_vector_name(
                DeleteVectorNameRequestBuilder::new("{collection_name}", "image").wait(true),
            )
            .await?;
        Ok(())
    }
    let _ = delete_vector_name().await;
}
