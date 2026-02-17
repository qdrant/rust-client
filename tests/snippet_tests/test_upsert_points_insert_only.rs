
#[tokio::test]
async fn test_upsert_points_insert_only() {
    async fn upsert_points_insert_only() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/upsert_points_insert_only.rs` file
        use qdrant_client::qdrant::{PointStruct, UpdateMode, UpsertPointsBuilder};
        use qdrant_client::{Payload, Qdrant};
        use serde_json::json;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let points = vec![PointStruct::new(
            1,
            vec![0.05, 0.61, 0.76, 0.74],
            Payload::try_from(json!({
                "city": "Berlin",
                "price": 1.99,
            }))
            .unwrap(),
        )];
        
        // Only insert new points, do not update existing ones
        client
            .upsert_points(
                UpsertPointsBuilder::new("{collection_name}", points)
                    .wait(true)
                    .update_mode(UpdateMode::InsertOnly),
            )
            .await?;
        Ok(())
    }
    let _ = upsert_points_insert_only().await;
}
