use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client.delete_full_snapshot("{snapshot_name}").await?;
