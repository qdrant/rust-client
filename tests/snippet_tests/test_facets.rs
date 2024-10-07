
#[tokio::test]
async fn test_facets() {
    async fn facets() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/facets.rs` file
        use qdrant_client::qdrant::{Condition, FacetCountsBuilder, Filter};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .facet(
                 FacetCountsBuilder::new("world_data", "country")
                     .limit(10)
                     .filter(Filter::must(vec![Condition::matches(
                         "continent",
                         "Europe".to_string(),
                     )])),
             )
             .await?;
        Ok(())
    }
    let _ = facets().await;
}
