
#[tokio::test]
async fn test_search_point_groups() {
    async fn search_point_groups() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/search_point_groups.rs` file
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
        Ok(())
    }
    let _ = search_point_groups().await;
}
