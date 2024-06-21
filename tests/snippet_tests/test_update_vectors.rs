
#[tokio::test]
async fn test_update_vectors() {
    async fn update_vectors() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/update_vectors.rs` file
        use qdrant_client::qdrant::{PointVectors, UpdatePointVectorsBuilder};
        use qdrant_client::Qdrant;
        use std::collections::HashMap;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .update_vectors(
                UpdatePointVectorsBuilder::new(
                    "{collection_name}",
                    vec![
                        PointVectors {
                            id: Some(1.into()),
                            vectors: Some(
                                HashMap::from([("image".to_string(), vec![0.1, 0.2, 0.3, 0.4])])
                                    .into(),
                            ),
                        },
                        PointVectors {
                            id: Some(2.into()),
                            vectors: Some(
                                HashMap::from([(
                                    "text".to_string(),
                                    vec![0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2],
                                )])
                                .into(),
                            ),
                        },
                    ],
                )
                .wait(true),
            )
            .await?;
        Ok(())
    }
    let _ = update_vectors().await;
}
