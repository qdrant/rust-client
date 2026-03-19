
#[tokio::test]
async fn test_create_snapshot() {
    async fn create_snapshot() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.create_snapshot("{collection_name}").await?;
        Ok(())
    }
    let _ = create_snapshot().await;
}
