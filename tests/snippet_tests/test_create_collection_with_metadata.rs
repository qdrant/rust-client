
#[tokio::test]
async fn test_create_collection_with_metadata() {
    async fn create_collection_with_metadata() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_collection_with_metadata.rs` file
        use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};
        use qdrant_client::Qdrant;
        use serde_json::{json, Value};
        use std::collections::HashMap;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        
        let mut metadata: HashMap<String, Value> = HashMap::new();
        metadata.insert("my-metadata-field".to_string(), json!("value-1"));
        metadata.insert("another-field".to_string(), json!(123));
        
        
        client
            .create_collection(
                CreateCollectionBuilder::new("{collection_name}")
                    .vectors_config(VectorParamsBuilder::new(100, Distance::Cosine))
                    .metadata(metadata),
            )
            .await?;
        Ok(())
    }
    let _ = create_collection_with_metadata().await;
}
