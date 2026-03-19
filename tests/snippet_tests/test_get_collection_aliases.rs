
#[tokio::test]
async fn test_get_collection_aliases() {
    async fn get_collection_aliases() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.list_collection_aliases("{collection_name}").await?;
        Ok(())
    }
    let _ = get_collection_aliases().await;
}
