
#[tokio::test]
async fn test_query_document() {
    async fn query_document() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/query_document.rs` file
        use qdrant_client::qdrant::{Document, Query, QueryPointsBuilder};
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let query_document = Document::new(
            "my query text",
            "sentence-transformers/all-minilm-l6-v2"
        );
        
        let query_request = QueryPointsBuilder::new("{collection_name}")
            .query(Query::new_nearest(query_document));
        
        // ANN search with server-side inference
        client.query(query_request).await?;
        
        Ok(())
    }
    let _ = query_document().await;
}
