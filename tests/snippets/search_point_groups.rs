use qdrant_client::qdrant::SearchPointGroupsBuilder;
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .search_groups(SearchPointGroupsBuilder::new(
        "{collection_name}",
        vec![1.1],
        4,
        "document_id",
        2,
    ))
    .await?;
