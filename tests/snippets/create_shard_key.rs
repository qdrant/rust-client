// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::{client::QdrantClient, qdrant::shard_key::Key};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .create_shard_key(
        "{collection_name}",
        &Key::Keyword("{shard_key".to_string()),
        None,
        None,
        &[],
    )
    .await?;
