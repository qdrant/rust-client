
#[tokio::test]
async fn test_create_collection_with_id_tracker() {
    async fn create_collection_with_id_tracker() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateCollectionBuilder, Distance, IdTrackerParamsBuilder, Memory, VectorParamsBuilder,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .create_collection(
                CreateCollectionBuilder::new("{collection_name}")
                    .vectors_config(VectorParamsBuilder::new(1536, Distance::Cosine))
                    // Keep the point id mapping of indexed segments in RAM
                    .id_tracker(IdTrackerParamsBuilder::default().memory(Memory::Pinned)),
            )
            .await?;
        Ok(())
    }
    let _ = create_collection_with_id_tracker().await;
}
