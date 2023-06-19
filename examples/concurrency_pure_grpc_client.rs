use anyhow::Result;
use qdrant_client::qdrant::collections_client::CollectionsClient;
use qdrant_client::qdrant::ListCollectionsRequest;
use std::time::Instant;
use tokio::task::JoinSet;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let connections_client = CollectionsClient::connect("http://localhost:6334").await?;
    let mut join_set = JoinSet::new();
    let now = Instant::now();

    for _ in 0..1000 {
        let mut client = connections_client.clone();
        join_set.spawn(async move {
            client
                .list(tonic::Request::new(ListCollectionsRequest {}))
                .await
        });
    }
    while let Some(res) = join_set.join_next().await {
        if let Ok(Ok(_)) = res {}
    }
    println!("Time elapse:{:?} for 1000 search", now.elapsed());

    Ok(())
}
