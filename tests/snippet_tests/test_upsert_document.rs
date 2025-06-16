
#[tokio::test]
async fn test_upsert_document() {
    async fn upsert_document() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/upsert_document.rs` file
        use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder, Document};
        use qdrant_client::{Qdrant, Payload};
        use serde_json::json;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let document = Document::new("my document", "sentence-transformers/all-minilm-l6-v2");
        
        client
            .upsert_points(
                UpsertPointsBuilder::new(
                    "{collection_name}",
                    vec![
                        PointStruct::new(
                            1,
                            document,
                            Payload::try_from(json!(
                                {"color": "red"}
                            ))
                            .unwrap(),
                        )
                    ],
                )
                .wait(true),
            )
            .await?;
        Ok(())
    }
    let _ = upsert_document().await;
}
