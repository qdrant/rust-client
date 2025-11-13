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
