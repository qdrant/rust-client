use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .scroll(
        ScrollPointsBuilder::new("{collection_name}")
            .filter(Filter::must([Condition::matches(
                "color",
                "red".to_string(),
            )]))
            .limit(1)
            .with_payload(true)
            .with_vectors(false),
    )
    .await?;

// Filter points matching any of the text words
client
    .scroll(
        ScrollPointsBuilder::new("{collection_name}")
            .filter(Filter::must([Condition::matches_text_any(
                "description",
                "machine learning artificial intelligence",
            )]))
            .limit(10u32),
    )
    .await?;
