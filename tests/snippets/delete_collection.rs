use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client.delete_collection("{collection_name}").await?;
