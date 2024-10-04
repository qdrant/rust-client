use qdrant_client::qdrant::{Condition, FacetCountsBuilder, Filter};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

let ten_countries_with_most_poins_in_europe = client
    .facet(
         FacetCountsBuilder::new("world_data", "country")
             .limit(10)
             .filter(Filter::must(vec![Condition::matches(
                 "continent",
                 "Europe".to_string(),
             )])),
     )
     .await?;
