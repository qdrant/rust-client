crate::qdrant_test_snippet!({
    use std::collections::HashMap;

    use qdrant_client::qdrant::UpdateCollectionBuilder;
    use qdrant_client::Qdrant;
    use serde_json::{json, Value};

    let client = Qdrant::from_url("http://localhost:6334").build()?;

    let mut metadata: HashMap<String, Value> = HashMap::new();
    metadata.insert(
        "my-metadata-field".to_string(),
        json!({
            "key-a": "value-a",
            "key-b": 42
        }),
    );

    client
        .update_collection(UpdateCollectionBuilder::new("{collection_name}").metadata(metadata))
        .await?;
});
