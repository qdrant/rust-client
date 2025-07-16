
#[tokio::test]
async fn test_create_collection_with_bq() {
    async fn create_collection_with_bq() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_collection_with_bq.rs` file
        use qdrant_client::qdrant::{
            BinaryQuantizationBuilder, BinaryQuantizationEncoding, CreateCollectionBuilder, Distance, VectorParamsBuilder, BinaryQuantizationQueryEncoding,
        };
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        client
            .create_collection(
                CreateCollectionBuilder::new("{collection_name}")
                    .vectors_config(VectorParamsBuilder::new(1536, Distance::Cosine))
                    .quantization_config(
                        BinaryQuantizationBuilder::new(true)
                            .encoding(BinaryQuantizationEncoding::TwoBits)
                            .query_encoding(BinaryQuantizationQueryEncoding::scalar8bits())
                    ),
            )
            .await?;
        Ok(())
    }
    let _ = create_collection_with_bq().await;
}
