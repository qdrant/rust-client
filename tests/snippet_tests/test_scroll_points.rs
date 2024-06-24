
#[tokio::test]
async fn test_scroll_points() {
    async fn scroll_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/scroll_points.rs` file
        use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .scroll(
                ScrollPointsBuilder::new("{collection_name}")
                    .filter(Filter::must([Condition::matches(
                        "color",
                        "red".to_string(),
                    )]))
                    .limit(1)
                    .with_payload(true)
                    .with_vectors(false),
            )
            .await?;
        Ok(())
    }
    let _ = scroll_points().await;
}
