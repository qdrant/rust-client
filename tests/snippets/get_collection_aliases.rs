use qdrant_client::client::QdrantClient;

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client.list_collection_aliases("{collection_name}").await?;
