
#[tokio::test]
async fn test_update_collection_memory() {
    async fn update_collection_memory() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            vectors_config_diff, CollectionParamsDiffBuilder, Memory, PayloadStorageParamsBuilder,
            UpdateCollectionBuilder, VectorParamsDiffBuilder,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .update_collection(
                UpdateCollectionBuilder::new("{collection_name}")
                    // Move the payload storage into RAM
                    .params(
                        CollectionParamsDiffBuilder::default()
                            .payload(PayloadStorageParamsBuilder::default().memory(Memory::Cached)),
                    )
                    // Move the original vectors to disk
                    .vectors_config(vectors_config_diff::Config::from(
                        VectorParamsDiffBuilder::default().memory(Memory::Cold),
                    )),
            )
            .await?;
        Ok(())
    }
    let _ = update_collection_memory().await;
}
