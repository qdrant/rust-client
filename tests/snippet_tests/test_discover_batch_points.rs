
#[tokio::test]
async fn test_discover_batch_points() {
    async fn discover_batch_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/discover_batch_points.rs` file
        use qdrant_client::qdrant::{
            vector_example::Example, ContextExamplePairBuilder, DiscoverBatchPointsBuilder,
            DiscoverPointsBuilder,
        };
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let discover_points = DiscoverBatchPointsBuilder::new(
            "{collection_name}",
            vec![
                DiscoverPointsBuilder::new(
                    "{collection_name}",
                    vec![
                        ContextExamplePairBuilder::default()
                            .positive(Example::Id(100.into()))
                            .negative(Example::Id(718.into()))
                            .build(),
                        ContextExamplePairBuilder::default()
                            .positive(Example::Id(200.into()))
                            .negative(Example::Id(300.into()))
                            .build(),
                    ],
                    10,
                )
                .build(),
                DiscoverPointsBuilder::new(
                    "{collection_name}",
                    vec![
                        ContextExamplePairBuilder::default()
                            .positive(Example::Id(342.into()))
                            .negative(Example::Id(213.into()))
                            .build(),
                        ContextExamplePairBuilder::default()
                            .positive(Example::Id(100.into()))
                            .negative(Example::Id(200.into()))
                            .build(),
                    ],
                    10,
                )
                .build(),
            ],
        );
        
        client.discover_batch(&discover_points.build()).await?;
        Ok(())
    }
    let _ = discover_batch_points().await;
}
