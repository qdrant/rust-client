
#[tokio::test]
async fn test_query_points_with_idf_corpus() {
    async fn query_points_with_idf_corpus() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            Condition, Filter, IdfParams, QueryPointsBuilder, SearchParamsBuilder,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        // Compute IDF statistics over the points of a single tenant instead of
        // the whole collection. Only applies to sparse vectors with the IDF modifier.
        client
            .query(
                QueryPointsBuilder::new("{collection_name}")
                    .query(vec![(1, 0.22), (42, 0.8)])
                    .using("sparse")
                    .filter(Filter::must([Condition::matches(
                        "tenant_id",
                        "tenant_1".to_string(),
                    )]))
                    .params(SearchParamsBuilder::default().idf(IdfParams::from(
                        Filter::must([Condition::matches("tenant_id", "tenant_1".to_string())]),
                    )))
                    .limit(10u64),
            )
            .await?;
        Ok(())
    }
    let _ = query_points_with_idf_corpus().await;
}
