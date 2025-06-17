use qdrant_client::qdrant::{Image, Query, QueryPointsBuilder};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

let query_image = Image::new_from_url(
    "https://picsum.photos/200/300.jpg",
    "Qdrant/clip-ViT-B-32-vision"
);

let query_request = QueryPointsBuilder::new("{collection_name}")
    .query(Query::new_nearest(query_image));

// ANN search with server-side inference
client.query(query_request).await?;

