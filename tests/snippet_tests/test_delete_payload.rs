
#[tokio::test]
async fn test_delete_payload() {
    async fn delete_payload() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_payload.rs` file
        use qdrant_client::qdrant::{DeletePayloadPointsBuilder, PointsIdsList};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .delete_payload(
                DeletePayloadPointsBuilder::new(
                    "{collection_name}",
                    vec!["color".to_string(), "price".to_string()],
                )
                .points_selector(PointsIdsList {
                    ids: vec![0.into(), 3.into(), 100.into()],
                })
                .wait(true),
            )
            .await?;
        Ok(())
    }
    let _ = delete_payload().await;
}
