// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::client::QdrantClient;

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .delete_field_index("{collection_name}", "{field_name}", None)
    .await?;
