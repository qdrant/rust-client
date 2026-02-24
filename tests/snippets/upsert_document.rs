crate::qdrant_test_snippet!({
    use qdrant_client::qdrant::{Document, PointStruct, UpsertPointsBuilder};
    use qdrant_client::{Payload, Qdrant};
    use serde_json::json;

    let client = Qdrant::from_url("http://localhost:6334").build()?;

    let document = Document::new("my document", "sentence-transformers/all-minilm-l6-v2");

    client
        .upsert_points(
            UpsertPointsBuilder::new(
                "{collection_name}",
                vec![PointStruct::new(
                    1,
                    document,
                    Payload::try_from(json!(
                        {"color": "red"}
                    ))
                    .unwrap(),
                )],
            )
            .wait(true),
        )
        .await?;
});
