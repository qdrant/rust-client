
#[tokio::test]
async fn test_search_points() {
    async fn search_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/search_points.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::{
            client::QdrantClient,
            qdrant::{Condition, Filter, SearchParams, SearchPoints},
        };
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .search_points(&SearchPoints {
                collection_name: "{collection_name}".to_string(),
                filter: Some(Filter::must([Condition::matches(
                    "city",
                    "London".to_string(),
                )])),
                params: Some(SearchParams {
                    hnsw_ef: Some(128),
                    exact: Some(false),
                    ..Default::default()
                }),
                vector: vec![0.2, 0.1, 0.9, 0.7],
                limit: 3,
                ..Default::default()
            })
            .await?;
        Ok(())
    }
    let _ = search_points().await;
}
