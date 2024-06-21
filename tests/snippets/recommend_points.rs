use qdrant_client::qdrant::{Condition, Filter, RecommendPointsBuilder, RecommendStrategy};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .recommend(
        RecommendPointsBuilder::new("{collection_name}", 3)
            .add_positive(100)
            .add_positive(200)
            .add_positive(vec![100.0, 231.0])
            .add_negative(718)
            .add_negative(vec![0.2, 0.3, 0.4, 0.5])
            .strategy(RecommendStrategy::AverageVector)
            .filter(Filter::must([Condition::matches(
                "city",
                "London".to_string(),
            )])),
    )
    .await?;
