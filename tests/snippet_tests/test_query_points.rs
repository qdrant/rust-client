
#[tokio::test]
async fn test_query_points() {
    async fn query_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/query_points.rs` file
        use qdrant_client::qdrant::{Fusion, PointId, PrefetchQueryBuilder, Query, QueryPointsBuilder, RecommendInputBuilder};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        // Query nearest by ID
        client.query(
            QueryPointsBuilder::new("{collection_name}")
                .query(PointId::from("43cf51e2-8777-4f52-bc74-c2cbde0c8b04"))
        ).await?;
        
        // Recommend on the average of these vectors
        client.query(
            QueryPointsBuilder::new("{collection_name}")
                .query(Query::new_recommend(
                    RecommendInputBuilder::default()
                        .add_positive(vec![0.1; 8])
                        .add_negative(PointId::from(0))
                ))
        ).await?;
        
        // Fusion query
        client.query(
            QueryPointsBuilder::new("{collection_name}")
                .add_prefetch(PrefetchQueryBuilder::default()
                    .query(vec![(1, 0.22), (42, 0.8)])
                    .using("sparse")
                    .limit(20u64)
                )
                .add_prefetch(PrefetchQueryBuilder::default()
                    .query(vec![0.01, 0.45, 0.67])
                    .using("dense")
                    .limit(20u64)
                )
                .query(Fusion::Rrf)
        ).await?;
        
        // 2-stage query
        client.query(
            QueryPointsBuilder::new("{collection_name}")
                .add_prefetch(PrefetchQueryBuilder::default()
                    .query(vec![0.01, 0.45, 0.67])
                    .limit(100u64)
                )
                .query(vec![
                    vec![0.1, 0.2],
                    vec![0.2, 0.1],
                    vec![0.8, 0.9],
                ])
                .using("colbert")
                .limit(10u64)
        ).await?;
        Ok(())
    }
    let _ = query_points().await;
}
