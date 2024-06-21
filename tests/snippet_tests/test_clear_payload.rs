
#[tokio::test]
async fn test_clear_payload() {
    async fn clear_payload() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/clear_payload.rs` file
        use qdrant_client::qdrant::{ClearPayloadPointsBuilder, PointsIdsList};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .clear_payload(ClearPayloadPointsBuilder::new("{collection_name}").points(
                PointsIdsList {
                    ids: vec![0.into(), 3.into(), 100.into()],
                },
            ))
            .await?;
        Ok(())
    }
    let _ = clear_payload().await;
}
