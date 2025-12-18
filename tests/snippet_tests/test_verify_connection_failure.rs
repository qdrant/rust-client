
#[tokio::test]
async fn test_verify_connection_failure() {
    async fn verify_connection_failure() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/verify_connection_failure.rs` file
        #[tokio::test]
        async fn test_verify_connection_failure() {
            use crate::qdrant_client::Qdrant;
        
            // We use an address that doesn't respond
            let client = Qdrant::from_url("http://127.0.0.1:1234")
                .connect_timeout(std::time::Duration::from_millis(500))
                .build()
                .unwrap();
        
            // build() always succeeds (lazy behavior),
            // but connect() should detect the failure.
            let result = client.connect().await;
            assert!(
                result.is_err(),
                "The connection should have failed on an invalid port"
            );
        }
        Ok(())
    }
    let _ = verify_connection_failure().await;
}
