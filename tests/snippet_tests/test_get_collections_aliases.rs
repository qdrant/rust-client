
#[tokio::test]
async fn test_get_collections_aliases() {
    async fn get_collections_aliases() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.list_aliases().await?;
        Ok(())
    }
    let _ = get_collections_aliases().await;
}
