
#[tokio::test]
async fn test_score_boost_closer_to_user() {
    async fn score_boost_closer_to_user() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/score_boost_closer_to_user.rs` file
        use qdrant_client::qdrant::{
            GeoPoint,  DecayParamsExpressionBuilder, Expression, FormulaBuilder, PrefetchQueryBuilder, QueryPointsBuilder,
        };
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .query(
                QueryPointsBuilder::new("{collection_name}")
                    .add_prefetch(
                        PrefetchQueryBuilder::default()
                            .query(vec![0.01, 0.45, 0.67])
                            .limit(100u64),
                    )
                    .query(
                        FormulaBuilder::new(Expression::sum_with([
                            Expression::variable("$score"),
                            Expression::exp_decay(
                                DecayParamsExpressionBuilder::new(Expression::geo_distance_with(
                                    GeoPoint {
                                        lat: 52.504043,
                                        lon: 13.393236,
                                    }, // Berlin
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
    let _ = score_boost_closer_to_user().await;
}
