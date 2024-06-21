use qdrant_client::qdrant::{CreateAliasBuilder, DeleteAlias};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .create_alias(CreateAliasBuilder::new(
        "example_collection",
        "production_collection",
    ))
    .await?;

client
    .delete_alias(DeleteAlias {
        alias_name: "production_collection".to_string(),
    })
    .await?;
