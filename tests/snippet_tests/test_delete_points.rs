
#[tokio::test]
async fn test_delete_points() {
    async fn delete_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_points.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::{client::QdrantClient, qdrant::{points_selector::PointsSelectorOneOf, Condition, Filter, PointsIdsList, PointsSelector}};
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .delete_points_blocking(
                "{collection_name}",
                None,
                &PointsSelector {
                    points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                        ids: vec![0.into(), 3.into(), 100.into()],
                    })),
                },
                None,
            )
            .await?;
        
            client
                .delete_points_blocking(
                    "{collection_name}",
                    None,
                    &PointsSelector {
                        points_selector_one_of: Some(PointsSelectorOneOf::Filter(Filter::must([
                            Condition::matches("color", "red".to_string()),
                        ]))),
                    },
                    None,
                )
                .await?;
        Ok(())
    }
    let _ = delete_points().await;
}
