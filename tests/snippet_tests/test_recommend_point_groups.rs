
#[tokio::test]
async fn test_recommend_point_groups() {
    async fn recommend_point_groups() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/recommend_point_groups.rs` file
        use qdrant_client::qdrant::{RecommendPointGroupsBuilder, RecommendStrategy};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .recommend_groups(
                RecommendPointGroupsBuilder::new("{collection_name}", "document_id", 2, 3)
                    .add_positive(100)
                    .add_positive(200)
                    .add_negative(718)
                    .strategy(RecommendStrategy::AverageVector),
            )
            .await?;
        Ok(())
    }
    let _ = recommend_point_groups().await;
}
