
#[tokio::test]
async fn test_set_payload() {
    async fn set_payload() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/set_payload.rs` file
        use qdrant_client::qdrant::{PointsIdsList, SetPayloadPointsBuilder};
        use qdrant_client::{Qdrant, Payload};
        use serde_json::json;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let payload: Payload = json!({
            "property1": "string",
            "property2": "string",
        })
        .try_into()
        .unwrap();
        
        client
            .set_payload(
                SetPayloadPointsBuilder::new("{collection_name}", payload)
                    .points_selector(PointsIdsList {
                        ids: vec![0.into(), 3.into(), 10.into()],
                    })
                    .wait(true),
            )
            .await?;
        Ok(())
    }
    let _ = set_payload().await;
}
