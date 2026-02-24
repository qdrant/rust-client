crate::qdrant_test_snippet!({
    use qdrant_client::qdrant::{Image, PointStruct, UpsertPointsBuilder};
    use qdrant_client::{Payload, Qdrant};
    use serde_json::json;

    let client = Qdrant::from_url("http://localhost:6334").build()?;

    let image = Image::new_from_url(
        "https://picsum.photos/200/300.jpg",
        "Qdrant/clip-ViT-B-32-vision",
    );

    client
        .upsert_points(
            UpsertPointsBuilder::new(
                "{collection_name}",
                vec![PointStruct::new(
                    1,
                    image,
                    Payload::try_from(json!(
                        {"color": "red"}
                    ))
                    .unwrap(),
                )],
            )
            .wait(true),
        )
        .await?;
});
