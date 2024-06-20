
#[tokio::test]
async fn test_clear_payload() {
    async fn clear_payload() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/clear_payload.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::{client::QdrantClient, qdrant::{
            points_selector::PointsSelectorOneOf, PointsIdsList, PointsSelector,
        }};
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .clear_payload(
                "{collection_name}",
                None,
                Some(PointsSelector {
                    points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                        ids: vec![0.into(), 3.into(), 100.into()],
                    })),
                }),
                None,
            )
            .await?;
        Ok(())
    }
    let _ = clear_payload().await;
}
