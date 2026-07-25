
#[tokio::test]
async fn test_create_collection_with_memory() {
    async fn create_collection_with_memory() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateCollectionBuilder, Distance, HnswConfigDiffBuilder, Memory,
            PayloadStorageParamsBuilder, ScalarQuantizationBuilder, VectorParamsBuilder,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .create_collection(
                CreateCollectionBuilder::new("{collection_name}")
                    .vectors_config(
                        VectorParamsBuilder::new(1536, Distance::Cosine)
                            // Keep the original vectors on disk, cached with usage
                            .memory(Memory::Cold),
                    )
                    .hnsw_config(
                        // Pre-load the HNSW graph into disk-cache RAM on start
                        HnswConfigDiffBuilder::default().memory(Memory::Cached),
                    )
                    .quantization_config(
                        // Keep quantized vectors in RAM and never evict them
                        ScalarQuantizationBuilder::default().memory(Memory::Pinned),
                    )
                    // Serve the payload from disk
                    .payload(PayloadStorageParamsBuilder::default().memory(Memory::Cold)),
            )
            .await?;
        Ok(())
    }
    let _ = create_collection_with_memory().await;
}
