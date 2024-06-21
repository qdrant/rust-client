
#[tokio::test]
async fn test_get_points() {
    async fn get_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/get_points.rs` file
        use qdrant_client::qdrant::GetPointsBuilder;
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .get_points(GetPointsBuilder::new(
                "{collection_name}",
                vec![0.into(), 30.into(), 100.into()],
            ))
            .await?;
        Ok(())
    }
    let _ = get_points().await;
}
