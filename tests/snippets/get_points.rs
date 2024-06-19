use qdrant_client::client::QdrantClient;

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .get_points(
        "{collection_name}",
        None,
        &[0.into(), 30.into(), 100.into()],
        Some(false),
        Some(false),
        None,
    )
    .await?;
