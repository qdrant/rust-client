
#[tokio::test]
async fn test_create_full_snapshot() {
    async fn create_full_snapshot() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.create_full_snapshot().await?;
        Ok(())
    }
    let _ = create_full_snapshot().await;
}
