
#[tokio::test]
async fn test_scroll_points_with_substring_match() {
    async fn scroll_points_with_substring_match() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder};
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .scroll(
                ScrollPointsBuilder::new("{collection_name}")
                    .filter(Filter::must([Condition::matches_substring("city", "erl")]))
                    .limit(10),
            )
            .await?;
        Ok(())
    }
    let _ = scroll_points_with_substring_match().await;
}
