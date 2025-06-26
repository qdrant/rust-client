
#[tokio::test]
async fn test_upsert_image() {
    async fn upsert_image() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/upsert_image.rs` file
        use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder, Image};
        use qdrant_client::{Qdrant, Payload};
        use serde_json::json;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let image = Image::new_from_url("https://picsum.photos/200/300.jpg", "Qdrant/clip-ViT-B-32-vision");
        
        client
            .upsert_points(
                UpsertPointsBuilder::new(
                    "{collection_name}",
                    vec![
                        PointStruct::new(
                            1,
                            image,
                            Payload::try_from(json!(
                                {"color": "red"}
                            ))
                            .unwrap(),
                        )
                    ],
                )
                .wait(true),
            )
            .await?;
        Ok(())
    }
    let _ = upsert_image().await;
}
