use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::with_payload_selector::SelectorOptions;
use qdrant_client::qdrant::SearchPoints;
use std::time::Instant;
use tokio::task::JoinSet;
#[tokio::main]
async fn main() -> Result<()> {
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config)).await?;
    let collection_name = "test";
    search_1000(client.clone(), collection_name.into()).await?;
    search_1000_concurrent(client, collection_name.into()).await?;
    Ok(())
}
pub fn get_search_points(collection_name: String) -> SearchPoints {
    SearchPoints {
        collection_name,
        vector: vec![1., 2., 3., 4., 7.],
        filter: None,
        limit: 5,
        with_vectors: None,
        with_payload: Some(qdrant_client::qdrant::WithPayloadSelector {
            selector_options: Some(SelectorOptions::Enable(true)),
        }),
        params: None,
        score_threshold: None,
        offset: None,
        ..Default::default()
    }
}
pub async fn search_1000(
    client: QdrantClient,
    collection_name: String,
) -> Result<(), anyhow::Error> {
    let now = Instant::now();
    for _ in 0..1000 {
        let search_points = get_search_points(collection_name.clone());
        client
            .search_points(&search_points)
            .await
            .map_or(Default::default(), |r| r);
    }
    println!("Time elapse:{:?} for 1000 search", now.elapsed());
    Ok(())
}
pub async fn search_1000_concurrent(
    client: QdrantClient,
    collection_name: String,
) -> Result<(), anyhow::Error> {
    let now = Instant::now();
    let mut join_set = JoinSet::new();

    for _ in 0..1000 {
        let client_cloned = client.clone();
        let collection_name_cloned = collection_name.clone();
        join_set.spawn(async move {
            let search_points = get_search_points(collection_name_cloned.clone());
            client_cloned.search_points(&search_points).await
        });
    }
    while let Some(res) = join_set.join_next().await {
        if let Ok(Ok(_)) = res {}
    }
    println!("Time elapse:{:?} for 1000 concurrent search", now.elapsed());
    Ok(())
}
