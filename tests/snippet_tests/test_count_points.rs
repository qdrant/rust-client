
#[tokio::test]
async fn test_count_points() {
    async fn count_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/count_points.rs` file
        use qdrant_client::qdrant::{Condition, CountPointsBuilder, Filter};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .count(
                CountPointsBuilder::new("{collection_name}")
                    .filter(Filter::must([Condition::matches(
                        "color",
                        "red".to_string(),
                    )]))
                    .exact(true),
            )
            .await?;
        Ok(())
    }
    let _ = count_points().await;
}
