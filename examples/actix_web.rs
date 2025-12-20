use std::sync::Arc;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use qdrant_client::Qdrant;
use serde_json::json;

struct AppState {
    client: Arc<Qdrant>,
}

async fn list_collections(data: web::Data<AppState>) -> impl Responder {
    match data.client.list_collections().await {
        Ok(collections) => {
            let names: Vec<String> = collections
                .collections
                .into_iter()
                .map(|c| c.name)
                .collect();
            HttpResponse::Ok().json(json!({ "collections": names }))
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn health_check(data: web::Data<AppState>) -> impl Responder {
    match data.client.health_check().await {
        Ok(resp) => HttpResponse::Ok().json(json!({ "result": format!("{:?}", resp) })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Qdrant::from_url("http://localhost:6334")
        .build()
        .expect("Failed to create Qdrant client");

    let state = web::Data::new(AppState {
        client: Arc::new(client),
    });

    println!("Starting Actix server on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/collections", web::get().to(list_collections))
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
