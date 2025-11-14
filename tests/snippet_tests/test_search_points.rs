
#[tokio::test]
async fn test_search_points() {
    async fn search_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/search_points.rs` file
        use qdrant_client::qdrant::{AcornSearchParamsBuilder, Condition, Filter, SearchParamsBuilder, SearchPointsBuilder, ShardKey, ShardKeySelectorBuilder};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .search_points(
                SearchPointsBuilder::new("{collection_name}", vec![0.2, 0.1, 0.9, 0.7], 3)
                    .filter(Filter::must([Condition::matches(
                        "city",
                        "London".to_string(),
                    )]))
                    .params(SearchParamsBuilder::default().hnsw_ef(128).exact(false)),
            )
            .await?;
        
        // Search with ACORN enabled for filtered search
        client
            .search_points(
                SearchPointsBuilder::new("{collection_name}", vec![0.2, 0.1, 0.9, 0.7], 3)
                    .filter(Filter::must([Condition::matches(
                        "city",
                        "London".to_string(),
                    )]))
                    .params(
                        SearchParamsBuilder::default()
                            .hnsw_ef(128)
                            .acorn(AcornSearchParamsBuilder::new(true).max_selectivity(0.4))
                    ),
            )
            .await?;
        
        // Search in specific shards with fallback
        client
            .search_points(
                SearchPointsBuilder::new("{collection_name}", vec![0.2, 0.1, 0.9, 0.7], 3)
                    .shard_key_selector(
                        ShardKeySelectorBuilder::with_shard_keys(vec![ShardKey::from("shard_1".to_string())])
                            .fallback(ShardKey::from("shard_backup".to_string()))
                    ),
            )
            .await?;
        Ok(())
    }
    let _ = search_points().await;
}
