
#[tokio::test]
async fn test_update_collection_metadata() {
    async fn update_collection_metadata() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/update_collection_metadata.rs` file
        use qdrant_client::qdrant::{UpdateCollectionBuilder};
        use qdrant_client::Qdrant;
        use serde_json::{json, Value};
        use std::collections::HashMap;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let mut metadata: HashMap<String, Value> = HashMap::new();
        metadata.insert("my-metadata-field".to_string(), json!({
            "key-a": "value-a",
            "key-b": 42
        }));
        
        client
            .update_collection(
                UpdateCollectionBuilder::new("{collection_name}").metadata(metadata),
            )
            .await?;
        Ok(())
    }
    let _ = update_collection_metadata().await;
}
