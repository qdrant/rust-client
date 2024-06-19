
#[tokio::test]
async fn test_get_points() {
    async fn get_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/get_points.rs` file
        use qdrant_client::client::QdrantClient;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .get_points(
                "{collection_name}",
                None,
                &[0.into(), 30.into(), 100.into()],
                Some(false),
                Some(false),
                None,
            )
            .await?;
        Ok(())
    }
    let _ = get_points().await;
}
