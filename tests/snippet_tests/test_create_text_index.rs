
#[tokio::test]
async fn test_create_text_index() {
    async fn create_text_index() -> Result<(), Box<dyn std::error::Error>> {
      // WARNING: This is a generated test snippet.
      // Please, modify the snippet in the `../snippets/create_text_index.rs` file
        use qdrant_client::qdrant::{
            CreateFieldIndexCollectionBuilder,
            TextIndexParamsBuilder,
            FieldType,
            TokenizerType,
        };
        use qdrant_client::Qdrant;
        
        let client = Qdrant::from_url("http://localhost:6334").build()?;
        
        let text_index_params = TextIndexParamsBuilder::new(TokenizerType::Word)
            .phrase_matching(true)
            .lowercase(true);
        
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
    let _ = create_text_index().await;
}
