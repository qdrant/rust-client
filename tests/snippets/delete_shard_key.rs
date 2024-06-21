use qdrant_client::qdrant::shard_key::Key;
use qdrant_client::qdrant::DeleteShardKeyRequestBuilder;
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .delete_shard_key(
        DeleteShardKeyRequestBuilder::new("{collection_name}")
            .key(Key::Keyword("{shard_key".to_string())),
    )
    .await?;
