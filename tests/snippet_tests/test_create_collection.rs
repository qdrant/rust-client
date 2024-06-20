
#[tokio::test]
async fn test_create_collection() {
    async fn create_collection() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_collection.rs` file
        // TODO: remove this once this test has been converted
        #![allow(deprecated)]
        
        use qdrant_client::{
            client::QdrantClient,
            qdrant::{vectors_config::Config, CreateCollection, Distance, VectorParams, VectorsConfig},
        };
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        client
            .create_collection(&CreateCollection {
                collection_name: "{collection_name}".to_string(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 100,
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            })
            .await?;
        Ok(())
    }
    let _ = create_collection().await;
}
