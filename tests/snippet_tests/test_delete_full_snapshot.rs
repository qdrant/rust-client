
#[tokio::test]
async fn test_delete_full_snapshot() {
    async fn delete_full_snapshot() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.delete_full_snapshot("{snapshot_name}").await?;
        Ok(())
    }
    let _ = delete_full_snapshot().await;
}
