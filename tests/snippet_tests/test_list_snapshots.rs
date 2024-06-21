
#[tokio::test]
async fn test_list_snapshots() {
    async fn list_snapshots() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/list_snapshots.rs` file
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.list_snapshots("{collection_name}").await?;
        Ok(())
    }
    let _ = list_snapshots().await;
}
