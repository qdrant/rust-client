
#[tokio::test]
async fn test_create_text_index_without_stemming() {
    async fn create_text_index_without_stemming() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateFieldIndexCollectionBuilder,
            FieldType,
            Memory,
            TextIndexParamsBuilder,
            TokenizerType,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        let text_index_params = TextIndexParamsBuilder::new(TokenizerType::Word)
            // Explicitly turn stemming off, overriding the language default
            .disabled_stemmer()
            .memory(Memory::Cold);

        client
            .create_field_index(
                CreateFieldIndexCollectionBuilder::new(
                    "{collection_name}",
                    "{field_name}",
                    FieldType::Text,
                ).field_index_params(text_index_params.build()),
            )
            .await?;
        Ok(())
    }
    let _ = create_text_index_without_stemming().await;
}
