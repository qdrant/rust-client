
#[tokio::test]
async fn test_update_collection_memory() {
    async fn update_collection_memory() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            vectors_config_diff, CollectionParamsDiffBuilder, Memory, PayloadStorageParams,
            StrictModeConfigBuilder, UpdateCollectionBuilder, VectorParamsDiffBuilder,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .update_collection(
                UpdateCollectionBuilder::new("{collection_name}")
                    // Move the payload storage into RAM
                    .params(
                        CollectionParamsDiffBuilder::default()
                            .payload(PayloadStorageParams::from(Memory::Cached)),
                    )
                    // Move the original vectors to disk
                    .vectors_config(vectors_config_diff::Config::from(
                        VectorParamsDiffBuilder::default().memory(Memory::Cold),
                    ))
                    // Reject updates once the storage filesystem is 90% full
                    .strict_mode_config(
                        StrictModeConfigBuilder::default()
                            .enabled(true)
                            .max_disk_usage_percent(90),
                    ),
            )
            .await?;
        Ok(())
    }
    let _ = update_collection_memory().await;
}
