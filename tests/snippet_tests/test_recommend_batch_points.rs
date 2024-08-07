
#[tokio::test]
async fn test_recommend_batch_points() {
    async fn recommend_batch_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/recommend_batch_points.rs` file
        use qdrant_client::qdrant::{
            Condition, Filter, RecommendBatchPointsBuilder, RecommendPointsBuilder,
        };
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let filter = Filter::must([Condition::matches("city", "London".to_string())]);
        let recommend_queries = vec![
            RecommendPointsBuilder::new("{collection_name}", 3)
                .add_positive(100)
                .add_positive(231)
                .add_negative(718)
                .filter(filter.clone())
                .build(),
            RecommendPointsBuilder::new("{collection_name}", 3)
                .add_positive(200)
                .add_positive(67)
                .add_negative(300)
                .filter(filter.clone())
                .build(),
        ];
        
        client
            .recommend_batch(RecommendBatchPointsBuilder::new(
                "{collection_name}",
                recommend_queries,
            ))
            .await?;
        Ok(())
    }
    let _ = recommend_batch_points().await;
}
