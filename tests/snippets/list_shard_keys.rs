use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

let _response = client.list_shard_keys("{collection_name}").await?;
