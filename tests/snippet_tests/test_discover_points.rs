
#[tokio::test]
async fn test_discover_points() {
    async fn discover_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/discover_points.rs` file
        use qdrant_client::qdrant::{
            target_vector::Target, vector_example::Example, ContextExamplePairBuilder,
            DiscoverPointsBuilder, VectorExample,
        };
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .discover(
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
                .target(Target::Single(VectorExample {
                    example: Some(Example::Vector(vec![0.2, 0.1, 0.9, 0.7].into())),
                })),
            )
            .await?;
        Ok(())
    }
    let _ = discover_points().await;
}
