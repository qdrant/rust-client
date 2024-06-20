// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::client::QdrantClient;

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client.create_alias("example_collection", "production_collection").await?;

client.delete_alias("production_collection").await?;
