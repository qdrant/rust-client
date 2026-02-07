#[macro_use]
extern crate rocket;
use qdrant_client::Qdrant;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};

#[get("/collections")]
async fn list_collections(
    client: &State<Qdrant>,
) -> Result<Json<Value>, status::Custom<Json<Value>>> {
    match client.list_collections().await {
        Ok(collections) => {
            let names: Vec<String> = collections
                .collections
                .into_iter()
                .map(|c| c.name)
                .collect();
            Ok(Json(json!({ "collections": names })))
        }
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

#[launch]
async fn rocket() -> _ {
    let client = Qdrant::from_url("http://localhost:6334")
        .build()
        .expect("Failed to create Qdrant client");

    rocket::build()
        .manage(client)
        .mount("/", routes![list_collections])
}
