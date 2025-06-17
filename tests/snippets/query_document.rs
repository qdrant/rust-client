use qdrant_client::qdrant::{Document, Query, QueryPointsBuilder};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

let query_document = Document::new(
    "my query text",
    "sentence-transformers/all-minilm-l6-v2"
);

let query_request = QueryPointsBuilder::new("{collection_name}")
    .query(Query::new_nearest(query_document));

// ANN search with server-side inference
client.query(query_request).await?;

