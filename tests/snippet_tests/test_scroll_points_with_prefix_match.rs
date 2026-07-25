
#[tokio::test]
async fn test_scroll_points_with_prefix_match() {
    async fn scroll_points_with_prefix_match() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder};
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        // Requires the keyword index of the field to be created with prefix matching enabled
        client
            .scroll(
                ScrollPointsBuilder::new("{collection_name}")
                    .filter(Filter::must([Condition::matches_prefix("city", "Ber")]))
                    .limit(10),
            )
            .await?;
        Ok(())
    }
    let _ = scroll_points_with_prefix_match().await;
}
