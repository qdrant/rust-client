use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

let scroll_response = client
    .scroll(
        ScrollPointsBuilder::new("{collection_name}")
            .filter(Filter::must([Condition::matches(
                "color",
                "red".to_string(),
            )]))
            .limit(1)
            .with_payload(true)
            .with_vectors(true),
    )
    .await?;

for point in scroll_response.result {
    let vector = point.vectors.unwrap().get_vector();
    println!("vector: {vector:?}");
}