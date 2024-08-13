
#[tokio::test]
async fn test_query_points_groups() {
    async fn query_points_groups() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/query_points_groups.rs` file
        use qdrant_client::Qdrant;
        use qdrant_client::qdrant::{Query, QueryPointGroupsBuilder};
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.query_groups(
            QueryPointGroupsBuilder::new("{collection_name}", "document_id")
                .query(Query::from(vec![0.01, 0.45, 0.67]))
                .limit(10u64)
                .group_size(5u64)
        ).await?;
        Ok(())
    }
    let _ = query_points_groups().await;
}
