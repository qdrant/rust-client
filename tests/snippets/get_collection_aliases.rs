use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client.list_collection_aliases("{collection_name}").await?;
