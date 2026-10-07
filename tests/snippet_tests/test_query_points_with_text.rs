
#[tokio::test]
async fn test_query_points_with_text() {
    async fn query_points_with_text() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{Condition, Filter, Query, QueryPointsBuilder, TextQueryBuilder};
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        // Rank by BM25 over the text index of the `{field_name}` payload field,
        // which must be created with BM25 scoring enabled
        client
            .query(
                QueryPointsBuilder::new("{collection_name}")
                    .query(Query::new_text("vector database"))
                    .using("{field_name}")
                    .limit(10),
            )
            .await?;

        // Tune BM25 parameters, required terms belong in the filter
        client
            .query(
                QueryPointsBuilder::new("{collection_name}")
                    .query(Query::new_text(
                        TextQueryBuilder::new("vector database").k(1.5).b(0.5),
                    ))
                    .using("{field_name}")
                    .filter(Filter::must([Condition::matches_text(
                        "{field_name}",
                        "rust",
                    )]))
                    .limit(10),
            )
            .await?;
        Ok(())
    }
    let _ = query_points_with_text().await;
}
