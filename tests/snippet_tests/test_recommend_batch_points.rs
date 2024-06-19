
#[tokio::test]
async fn test_recommend_batch_points() {
    async fn recommend_batch_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/recommend_batch_points.rs` file
        use qdrant_client::{
            client::QdrantClient,
            qdrant::{Condition, Filter, RecommendBatchPoints, RecommendPoints},
        };
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        let filter = Filter::must([Condition::matches("city", "London".to_string())]);
        
        let recommend_queries = vec![
            RecommendPoints {
                collection_name: "{collection_name}".to_string(),
                positive: vec![100.into(), 231.into()],
                negative: vec![718.into()],
                filter: Some(filter.clone()),
                limit: 3,
                ..Default::default()
            },
            RecommendPoints {
                collection_name: "{collection_name}".to_string(),
                positive: vec![200.into(), 67.into()],
                negative: vec![300.into()],
                filter: Some(filter),
                limit: 3,
                ..Default::default()
            },
        ];
        
        client
            .recommend_batch(&RecommendBatchPoints {
                collection_name: "{collection_name}".to_string(),
                recommend_points: recommend_queries,
                ..Default::default()
            })
            .await?;
        Ok(())
    }
    let _ = recommend_batch_points().await;
}
