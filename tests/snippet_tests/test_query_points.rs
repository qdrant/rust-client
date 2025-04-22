
#[tokio::test]
async fn test_query_points() {
    async fn query_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/query_points.rs` file
        use qdrant_client::qdrant::{
            Condition, DecayParamsExpressionBuilder, Expression, FormulaBuilder, Fusion, GeoPoint,
            PointId, PrefetchQueryBuilder, Query, QueryPointsBuilder, RecommendInputBuilder,
            Sample,
        };
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        // Query nearest by ID
        let _nearest = client.query(
            QueryPointsBuilder::new("{collection_name}")
                .query(PointId::from("43cf51e2-8777-4f52-bc74-c2cbde0c8b04"))
        ).await?;
        
        // Recommend on the average of these vectors
        let _recommendations = client.query(
            QueryPointsBuilder::new("{collection_name}")
                .query(Query::new_recommend(
                    RecommendInputBuilder::default()
                        .add_positive(vec![0.1; 8])
                        .add_negative(PointId::from(0))
                ))
        ).await?;
        
        // Fusion query
        let _hybrid = client.query(
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
        let _refined = client.query(
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
        
        // Random sampling (as of 1.11.0)
        let _sampled = client
            .query(
                QueryPointsBuilder::new("{collection_name}")
                    .query(Query::new_sample(Sample::Random))
            )
            .await?;
        
        // Score boost depending on payload conditions (as of 1.14.0)
        let _tag_boosted = client.query(
            QueryPointsBuilder::new("{collection_name}")
                .add_prefetch(PrefetchQueryBuilder::default()
                    .query(vec![0.01, 0.45, 0.67])
                    .limit(100u64)
                )
                .query(FormulaBuilder::new(Expression::sum_with([
                    Expression::score(),
                    Expression::mult_with([
                        Expression::constant(0.5),
                        Expression::condition(Condition::matches("tag", ["h1", "h2", "h3", "h4"])),
                    ]),
                    Expression::mult_with([
                        Expression::constant(0.25),
                        Expression::condition(Condition::matches("tag", ["p", "li"])),
                    ]),
                ])))
                .limit(10)
        ).await?;
        
        // Score boost geographically closer points (as of 1.14.0)
        let _geo_boosted = client.query(
            QueryPointsBuilder::new("{collection_name}")
                    .add_prefetch(
                        PrefetchQueryBuilder::default()
                            .query(vec![0.01, 0.45, 0.67])
                            .limit(100u64),
                    )
                    .query(
                        FormulaBuilder::new(Expression::sum_with([
                            Expression::score(),
                            Expression::exp_decay(
                                DecayParamsExpressionBuilder::new(Expression::geo_distance_with(
                                    // Berlin
                                    GeoPoint { lat: 52.504043, lon: 13.393236 },
                                    "geo.location",
                                ))
                                .scale(5_000.0),
                            ),
                        ]))
                        // Munich
                        .add_default("geo.location", GeoPoint { lat: 48.137154, lon: 11.576124 }),
                    )
                    .limit(10),
            )
            .await?;
        Ok(())
    }
    let _ = query_points().await;
}
