
#[tokio::test]
async fn test_collection_exists() {
    async fn collection_exists() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.collection_exists("{collection_name}").await?;
        Ok(())
    }
    let _ = collection_exists().await;
}
