use qdrant_client::{
    client::QdrantClient,
    qdrant::{
        target_vector::Target, vector_example::Example, ContextExamplePair, DiscoverPoints,
        TargetVector, VectorExample,
    },
};

let client = QdrantClient::from_url("http://localhost:6334").build()?;

client
    .discover(&DiscoverPoints {
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
    })
    .await?;
