use qdrant_client::qdrant::{
    points_selector::PointsSelectorOneOf,
    points_update_operation::{
        Operation, OverwritePayload, PointStructList, UpdateVectors,
    },
    PointStruct, PointVectors, PointsIdsList, PointsSelector, PointsUpdateOperation,
    UpdateBatchPointsBuilder,
};
use qdrant_client::{Qdrant, Payload};
use serde_json::json;
use std::collections::HashMap;

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
