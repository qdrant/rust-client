use qdrant_client::Qdrant;
use qdrant_client::qdrant::{Query, QueryPointGroupsBuilder};

let client = Qdrant::from_url("http://localhost:6334").build()?;

client.query_groups(
    QueryPointGroupsBuilder::new("{collection_name}", "document_id")
        .query(Query::from(vec![0.01, 0.45, 0.67]))
        .limit(10u64)
        .group_size(5u64)
).await?;
