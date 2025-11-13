
#[tokio::test]
async fn test_upsert_points_with_condition() {
    async fn upsert_points_with_condition() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/upsert_points_with_condition.rs` file
        use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder, Filter, Condition};
        use qdrant_client::{Payload, Qdrant};
        use serde_json::json;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let points = vec![
            PointStruct::new(
                1,
                vec![0.05, 0.61, 0.76, 0.74],
                Payload::try_from(json!({
                    "city": "Berlin", 
                    "price": 1.99,
                    "version": 3
                })).unwrap(),
            )
        ];
        
        client
            .upsert_points(
                UpsertPointsBuilder::new("{collection_name}", points)
                    .wait(true)
                    .update_filter(Filter::must([Condition::matches("version", 2)]))
            ).await?;
        Ok(())
    }
    let _ = upsert_points_with_condition().await;
}
