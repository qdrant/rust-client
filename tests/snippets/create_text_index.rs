use qdrant_client::qdrant::{
    CreateFieldIndexCollectionBuilder,
    TextIndexParamsBuilder,
    FieldType,
    TokenizerType,
};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

let text_index_params = TextIndexParamsBuilder::new(TokenizerType::Word)
    .min_token_len(2)
    .max_token_len(10)
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