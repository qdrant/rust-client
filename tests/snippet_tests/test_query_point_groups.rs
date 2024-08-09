
#[tokio::test]
async fn test_query_point_groups() {
    async fn query_point_groups() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/query_point_groups.rs` file
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
        Ok(())
    }
    let _ = query_point_groups().await;
}
