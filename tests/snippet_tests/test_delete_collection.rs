
#[tokio::test]
async fn test_delete_collection() {
    async fn delete_collection() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.delete_collection("{collection_name}").await?;
        Ok(())
    }
    let _ = delete_collection().await;
}
