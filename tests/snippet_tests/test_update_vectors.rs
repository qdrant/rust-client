
#[tokio::test]
async fn test_update_vectors() {
    async fn update_vectors() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/update_vectors.rs` file
        use qdrant_client::client::QdrantClient;
        use qdrant_client::qdrant::PointVectors;
        use std::collections::HashMap;
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .update_vectors_blocking(
                "{collection_name}",
                None,
                &[
                    PointVectors {
                        id: Some(1.into()),
                        vectors: Some(
                            HashMap::from([("image".to_string(), vec![0.1, 0.2, 0.3, 0.4])]).into(),
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
                None,
            )
            .await?;
        Ok(())
    }
    let _ = update_vectors().await;
}
