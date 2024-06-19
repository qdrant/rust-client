use qdrant_client::client::QdrantClient;

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client.delete_collection("{collection_name}").await?;
