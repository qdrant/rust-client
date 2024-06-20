
#[tokio::test]
async fn test_search_point_groups() {
    async fn search_point_groups() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/search_point_groups.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
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
        Ok(())
    }
    let _ = search_point_groups().await;
}
