
#[tokio::test]
async fn test_create_collection_with_turbo4_datatype() {
    async fn create_collection_with_turbo4_datatype() -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::{
            CreateCollectionBuilder, Datatype, Distance, VectorParamsBuilder,
        };
        use qdrant_client::Qdrant;

        let client = Qdrant::from_url("http://localhost:6334").build()?;

        client
            .create_collection(
                CreateCollectionBuilder::new("{collection_name}").vectors_config(
                    VectorParamsBuilder::new(1536, Distance::Cosine)
                        .datatype(Datatype::Turbo4),
                ),
            )
            .await?;
        Ok(())
    }
    let _ = create_collection_with_turbo4_datatype().await;
}
