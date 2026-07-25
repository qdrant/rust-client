
#[tokio::test]
async fn test_create_keyword_index_with_prefix() {
    async fn create_keyword_index_with_prefix() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateFieldIndexCollectionBuilder, FieldType, KeywordIndexParamsBuilder, Memory,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        let keyword_index_params = KeywordIndexParamsBuilder::default()
            // Allow `Condition::matches_prefix` on this field
            .prefix(true)
            .memory(Memory::Cached);

        client
            .create_field_index(
                CreateFieldIndexCollectionBuilder::new(
                    "{collection_name}",
                    "{field_name}",
                    FieldType::Keyword,
                ).field_index_params(keyword_index_params.build()),
            )
            .await?;
        Ok(())
    }
    let _ = create_keyword_index_with_prefix().await;
}
