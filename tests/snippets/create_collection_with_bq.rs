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