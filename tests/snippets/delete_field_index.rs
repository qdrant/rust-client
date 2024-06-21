use qdrant_client::qdrant::DeleteFieldIndexCollectionBuilder;
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .delete_field_index(DeleteFieldIndexCollectionBuilder::new(
        "{collection_name}",
        "{field_name}",
    ))
    .await?;
