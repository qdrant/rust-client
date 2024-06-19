
#[tokio::test]
async fn test_delete_vectors() {
    async fn delete_vectors() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/delete_vectors.rs` file
        use qdrant_client::{client::QdrantClient, qdrant::{
            points_selector::PointsSelectorOneOf, PointsIdsList, PointsSelector, VectorsSelector,
        }};
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .delete_vectors_blocking(
                "{collection_name}",
                None,
                &PointsSelector {
                    points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                        ids: vec![0.into(), 3.into(), 10.into()],
                    })),
                },
                &VectorsSelector {
                    names: vec!["text".into(), "image".into()],
                },
                None,
            )
            .await?;
        Ok(())
    }
    let _ = delete_vectors().await;
}
