
#[tokio::test]
async fn test_discover_batch_points() {
    async fn discover_batch_points() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/discover_batch_points.rs` file
        use qdrant_client::{
            client::QdrantClient,
            qdrant::{
                target_vector::Target, vector_example::Example, ContextExamplePair, DiscoverBatchPoints,
                DiscoverPoints, TargetVector, VectorExample,
            },
        };
        
        let client = QdrantClient::from_url("http://localhost:6334").build()?;
        
        let discover_points = &DiscoverBatchPoints {
            collection_name: "{collection_name}".to_string(),
            discover_points: vec![
                DiscoverPoints {
                    collection_name: "{collection_name}".to_string(),
                    target: Some(TargetVector {
                        target: Some(Target::Single(VectorExample {
                            example: Some(Example::Vector(vec![0.2, 0.1, 0.9, 0.7].into())),
                        })),
                    }),
                    context: vec![
                        ContextExamplePair {
                            positive: Some(VectorExample {
                                example: Some(Example::Id(100.into())),
                            }),
                            negative: Some(VectorExample {
                                example: Some(Example::Id(718.into())),
                            }),
                        },
                        ContextExamplePair {
                            positive: Some(VectorExample {
                                example: Some(Example::Id(200.into())),
                            }),
                            negative: Some(VectorExample {
                                example: Some(Example::Id(300.into())),
                            }),
                        },
                    ],
                    limit: 10,
                    ..Default::default()
                },
                DiscoverPoints {
                    collection_name: "{collection_name}".to_string(),
                    target: Some(TargetVector {
                        target: Some(Target::Single(VectorExample {
                            example: Some(Example::Vector(vec![0.5, 0.3, 0.2, 0.3].into())),
                        })),
                    }),
                    context: vec![
                        ContextExamplePair {
                            positive: Some(VectorExample {
                                example: Some(Example::Id(342.into())),
                            }),
                            negative: Some(VectorExample {
                                example: Some(Example::Id(213.into())),
                            }),
                        },
                        ContextExamplePair {
                            positive: Some(VectorExample {
                                example: Some(Example::Id(100.into())),
                            }),
                            negative: Some(VectorExample {
                                example: Some(Example::Id(200.into())),
                            }),
                        },
                    ],
                    limit: 10,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        
        client.discover_batch(discover_points).await?;
        Ok(())
    }
    let _ = discover_batch_points().await;
}
