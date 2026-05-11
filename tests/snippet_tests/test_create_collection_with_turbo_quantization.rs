
#[tokio::test]
async fn test_create_collection_with_turbo_quantization() {
    async fn create_collection_with_turbo_quantization() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateCollectionBuilder, Distance, TurboQuantBitSize, TurboQuantizationBuilder,
            VectorParamsBuilder,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .create_collection(
                CreateCollectionBuilder::new("{collection_name}")
                    .vectors_config(VectorParamsBuilder::new(1536, Distance::Cosine))
                    .quantization_config(
                        TurboQuantizationBuilder::new()
                            .bits(TurboQuantBitSize::Bits2)
                            .always_ram(true),
                    ),
            )
            .await?;
        Ok(())
    }
    let _ = create_collection_with_turbo_quantization().await;
}
