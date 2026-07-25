
#[tokio::test]
async fn test_scroll_points_with_slice() {
    async fn scroll_points_with_slice() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder};
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        // Split the id space into 4 disjoint slices, so they can be scrolled in parallel
        let total_slices = 4;

        for slice in 0..total_slices {
            client
                .scroll(
                    ScrollPointsBuilder::new("{collection_name}")
                        .filter(Filter::must([Condition::slice(total_slices, slice)]))
                        .limit(10),
                )
                .await?;
        }
        Ok(())
    }
    let _ = scroll_points_with_slice().await;
}
