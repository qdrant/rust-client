use qdrant_client::qdrant::{Condition, Filter, SearchParamsBuilder, SearchPointsBuilder};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client
    .search_points(
        SearchPointsBuilder::new("{collection_name}", vec![0.2, 0.1, 0.9, 0.7], 3)
            .filter(Filter::must([Condition::matches(
                "city",
                "London".to_string(),
            )]))
            .params(SearchParamsBuilder::default().hnsw_ef(128).exact(false)),
    )
    .await?;
