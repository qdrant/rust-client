
#[tokio::test]
async fn test_batch_update() {
    async fn batch_update() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/batch_update.rs` file
        use serde_json::json;
        use qdrant_client::client::{ QdrantClient, Payload };
        use std::collections::HashMap;
        use qdrant_client::qdrant::{
            points_selector::PointsSelectorOneOf,
            points_update_operation::{
                Operation, PointStructList, UpdateVectors, OverwritePayload
            },
            PointStruct, PointVectors, PointsIdsList, PointsSelector, PointsUpdateOperation,
        };
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .batch_updates_blocking(
                "{collection_name}",
                &[
                    PointsUpdateOperation {
                        operation: Some(Operation::Upsert(PointStructList {
                            points: vec![PointStruct::new(
                                1,
                                vec![1.0, 2.0, 3.0, 4.0],
                                Payload::try_from(json!({})).unwrap(),
                            )],
                            ..Default::default()
                        })),
                    },
                    PointsUpdateOperation {
                        operation: Some(Operation::UpdateVectors(UpdateVectors {
                            points: vec![PointVectors {
                                id: Some(1.into()),
                                vectors: Some(vec![1.0, 2.0, 3.0, 4.0].into()),
                            }],
                            ..Default::default()
                        })),
                    },
                    PointsUpdateOperation {
                        operation: Some(Operation::OverwritePayload(OverwritePayload {
                            points_selector: Some(PointsSelector {
                                points_selector_one_of: Some(PointsSelectorOneOf::Points(
                                    PointsIdsList {
                                        ids: vec![1.into()],
                                    },
                                )),
                            }),
                            payload: HashMap::from([("test_payload".to_string(), 1.into())]),
                            ..Default::default()
                        })),
                    },
                ],
                None,
            )
            .await?;
        Ok(())
    }
    let _ = batch_update().await;
}
