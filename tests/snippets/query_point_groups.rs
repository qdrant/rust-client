use qdrant_client::qdrant::QueryPointGroupsBuilder;
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .query_groups(QueryPointGroupsBuilder::new(
        "{collection_name}",
        "document_id",
    )
    .query(vec![1.1]))
    .await?;
