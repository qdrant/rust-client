use qdrant_client::client::QdrantClient;

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .delete_snapshot("{collection_name}", "{snapshot_name}")
    .await?;
