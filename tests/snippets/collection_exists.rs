// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::client::QdrantClient;

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client.collection_exists("{collection_name}").await?;
