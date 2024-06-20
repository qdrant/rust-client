// TODO: remove this once this test has been converted
#![allow(deprecated)]

use qdrant_client::{client::QdrantClient, qdrant::FieldType};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
.create_field_index(
    "{collection_name}",
    "{field_name}",
    FieldType::Keyword,
    None,
    None,
)
.await?;
