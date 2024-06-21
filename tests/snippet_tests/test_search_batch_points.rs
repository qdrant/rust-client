
#[tokio::test]
async fn test_search_batch_points() {
    async fn search_batch_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/search_batch_points.rs` file
        use qdrant_client::qdrant::{
            Condition, Filter, SearchBatchPointsBuilder, SearchPointsBuilder,
        };
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let filter = Filter::must([Condition::matches("city", "London".to_string())]);
        
        let searches = vec![
            SearchPointsBuilder::new("{collection_name}", vec![0.2, 0.1, 0.9, 0.7], 3)
                .filter(filter.clone())
                .build(),
            SearchPointsBuilder::new("{collection_name}", vec![0.5, 0.3, 0.2, 0.3], 3)
                .filter(filter.clone())
                .build(),
        ];
        
        client
            .search_batch_points(SearchBatchPointsBuilder::new("{collection_name}", searches))
            .await?;
        Ok(())
    }
    let _ = search_batch_points().await;
}
