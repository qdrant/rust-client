
#[tokio::test]
async fn test_count_points() {
    async fn count_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/count_points.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::{client::QdrantClient, qdrant::{Condition, CountPoints, Filter}};
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .count(&CountPoints {
                collection_name: "{collection_name}".to_string(),
                filter: Some(Filter::must([Condition::matches(
                    "color",
                    "red".to_string(),
                )])),
                exact: Some(true),
                ..Default::default()
            })
            .await?;
        Ok(())
    }
    let _ = count_points().await;
}
