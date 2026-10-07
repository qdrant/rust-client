
#[tokio::test]
async fn test_create_text_index_with_bm25() {
    async fn create_text_index_with_bm25() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateFieldIndexCollectionBuilder, FieldType, TextIndexParamsBuilder, TokenizerType,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        // Enable BM25 ranking over this field, implies phrase matching
        let text_index_params = TextIndexParamsBuilder::new(TokenizerType::Word)
            .lowercase(true)
            .bm25_scoring();

        client
            .create_field_index(
                CreateFieldIndexCollectionBuilder::new(
                    "{collection_name}",
                    "{field_name}",
                    FieldType::Text,
                )
                .field_index_params(text_index_params),
            )
            .await?;
        Ok(())
    }
    let _ = create_text_index_with_bm25().await;
}
