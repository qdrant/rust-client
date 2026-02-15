use qdrant_client::qdrant::{
    FeedbackItemBuilder, FeedbackStrategyBuilder, PointId, Query, QueryPointsBuilder,
    RelevanceFeedbackInputBuilder, VectorInput,
};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

// Relevance feedback query (as of 1.17.0)
let _feedback = client.query(
    QueryPointsBuilder::new("{collection_name}")
        .query(Query::new_relevance_feedback(
            RelevanceFeedbackInputBuilder::new(vec![0.01, 0.45, 0.67])
                .add_feedback(FeedbackItemBuilder::new(VectorInput::new_id(PointId::from(42)), 0.9))
                .add_feedback(FeedbackItemBuilder::new(VectorInput::new_id(PointId::from(7)), 0.1))
                .strategy(FeedbackStrategyBuilder::naive(1.0, 1.0, 1.0))
        ))
        .limit(10u64)
).await?;
