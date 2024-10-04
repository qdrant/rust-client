
#[tokio::test]
async fn test_search_matrix_pairs() {
    async fn search_matrix_pairs() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/search_matrix_pairs.rs` file
        use qdrant_client::qdrant::{Condition, SearchMatrixPointsBuilder, Filter};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let matrix = client
            .search_matrix_pairs(
                SearchMatrixPointsBuilder::new("collection_name")
                   .filter(Filter::must(vec![Condition::matches(
                       "color",
                       "red".to_string(),
                   )]))
                   .sample(1000)
                   .limit(10),
            )
            .await?;
        Ok(())
    }
    let _ = search_matrix_pairs().await;
}
