use std::sync::Arc;

use axum::routing::get;
use axum::{Json, Router};
use qdrant_client::Qdrant;
use serde_json::{json, Value};

async fn list_collections(
    axum::extract::State(client): axum::extract::State<Arc<Qdrant>>,
) -> Json<Value> {
    match client.list_collections().await {
        Ok(collections) => {
            let names: Vec<String> = collections
                .collections
                .into_iter()
                .map(|c| c.name)
                .collect();
            Json(json!({ "collections": names }))
        }
        Err(e) => Json(json!({"error": e.to_string()})),
    }
}

#[tokio::main]
async fn main() {
    let client = Qdrant::from_url("http://localhost:6334")
        .build()
        .expect("Failed to create Qdrant client");

    let app = Router::new()
        .route("/collections", get(list_collections))
        .with_state(Arc::new(client));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Starting Axum server on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
