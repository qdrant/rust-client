
#[tokio::test]
async fn test_list_full_snapshots() {
    async fn list_full_snapshots() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/list_full_snapshots.rs` file
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client.list_full_snapshots().await?;
        Ok(())
    }
    let _ = list_full_snapshots().await;
}
