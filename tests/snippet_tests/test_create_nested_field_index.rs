
#[tokio::test]
async fn test_create_nested_field_index() {
    async fn create_nested_field_index() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateFieldIndexCollectionBuilder, FieldType, FloatIndexParamsBuilder,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        // Index every element of `offers` separately, so a condition on
        // `offers[].price` matches a point only when a single offer matches
        client
            .create_field_index(
                CreateFieldIndexCollectionBuilder::new(
                    "{collection_name}",
                    "offers[].price",
                    FieldType::Float,
                )
                .field_index_params(FloatIndexParamsBuilder::default().nested("offers")),
            )
            .await?;
        Ok(())
    }
    let _ = create_nested_field_index().await;
}
