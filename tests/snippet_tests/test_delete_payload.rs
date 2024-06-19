
#[tokio::test]
async fn test_delete_payload() {
    async fn delete_payload() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_payload.rs` file
        use qdrant_client::{client::QdrantClient, qdrant::{
            points_selector::PointsSelectorOneOf, PointsIdsList, PointsSelector,
        }};
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .delete_payload_blocking(
                "{collection_name}",
                None,
                &PointsSelector {
                    points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                        ids: vec![0.into(), 3.into(), 100.into()],
                    })),
                },
                vec!["color".to_string(), "price".to_string()],
                None,
            )
            .await?;
        Ok(())
    }
    let _ = delete_payload().await;
}
