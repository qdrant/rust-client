crate::qdrant_test_snippet!({
    use std::collections::HashMap;

    use qdrant_client::qdrant::points_selector::PointsSelectorOneOf;
    use qdrant_client::qdrant::points_update_operation::{
        Operation, OverwritePayload, PointStructList, UpdateVectors,
    };
    use qdrant_client::qdrant::{
        PointStruct, PointVectors, PointsIdsList, PointsSelector, PointsUpdateOperation,
        UpdateBatchPointsBuilder,
    };
    use qdrant_client::{Payload, Qdrant};
    use serde_json::json;

    let client = Qdrant::from_url("http://localhost:6334").build()?;

    client
        .update_points_batch(
            UpdateBatchPointsBuilder::new(
                "{collection_name}",
                vec![
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
            )
            .wait(true),
        )
        .await?;
});
