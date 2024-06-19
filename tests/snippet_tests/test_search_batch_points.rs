
#[tokio::test]
async fn test_search_batch_points() {
    async fn search_batch_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/search_batch_points.rs` file
        use qdrant_client::{
            client::QdrantClient,
            qdrant::{Condition, Filter, SearchBatchPoints, SearchPoints},
        };
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        let filter = Filter::must([Condition::matches("city", "London".to_string())]);
        
        let searches = vec![
            SearchPoints {
                collection_name: "{collection_name}".to_string(),
                vector: vec![0.2, 0.1, 0.9, 0.7],
                filter: Some(filter.clone()),
                limit: 3,
                ..Default::default()
            },
            SearchPoints {
                collection_name: "{collection_name}".to_string(),
                vector: vec![0.5, 0.3, 0.2, 0.3],
                filter: Some(filter),
                limit: 3,
                ..Default::default()
            },
        ];
        
        client
            .search_batch_points(&SearchBatchPoints {
                collection_name: "{collection_name}".to_string(),
                search_points: searches,
                read_consistency: None,
                ..Default::default()
            })
            .await?;
        Ok(())
    }
    let _ = search_batch_points().await;
}
