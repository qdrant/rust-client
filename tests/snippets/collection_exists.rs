use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client.collection_exists("{collection_name}").await?;
