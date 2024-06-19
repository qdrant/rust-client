use qdrant_client::{
    client::QdrantClient,
    qdrant::SearchPointGroups,
};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .search_groups(&SearchPointGroups {
        collection_name: "{collection_name}".to_string(),
        vector: vec![1.1],
        group_by: "document_id".to_string(),
        limit: 4,
        group_size: 2,
        ..Default::default()
    })
    .await?;
