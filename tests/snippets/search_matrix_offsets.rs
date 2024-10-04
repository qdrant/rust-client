use qdrant_client::qdrant::{Condition, SearchMatrixPointsBuilder, Filter};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

let matrix = client
    .search_matrix_offsets(
        SearchMatrixPointsBuilder::new("collection_name")
           .filter(Filter::must(vec![Condition::matches(
               "color",
               "red".to_string(),
           )]))
           .sample(1000)
           .limit(10),
    )
    .await?;
