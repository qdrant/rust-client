
#[tokio::test]
async fn test_with_header() {
    async fn with_header() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/with_header.rs` file
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334")
            .header("x-base", "base-value")
            .skip_compatibility_check()
            .build()
            .unwrap();
        
        let traced = client.with_header("x-request-id", "abc-123");
        
        assert_eq!(
            traced.config.custom_headers,
            vec![
                ("x-base".to_string(), "base-value".to_string()),
                ("x-request-id".to_string(), "abc-123".to_string()),
            ]
        );
        // Original still has only the base header
        assert_eq!(
            client.config.custom_headers,
            vec![("x-base".to_string(), "base-value".to_string())]
        );
        Ok(())
    }
    let _ = with_header().await;
}
