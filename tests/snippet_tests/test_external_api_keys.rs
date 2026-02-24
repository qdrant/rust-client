use std::collections::HashMap;

use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, Document, PointStruct, Query, QueryPointsBuilder,
    UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant};
use serde_json::json;

const PROXY_URL: &str = "http://localhost:6334";
const UPSERT_COLLECTION_NAME: &str = "test_external_api_keys_upsert";
const QUERY_COLLECTION_NAME: &str = "test_external_api_keys_query";
const DUAL_OPENAI_COLLECTION_NAME: &str = "test_external_api_keys_dual_openai";
const DUAL_COHERE_COLLECTION_NAME: &str = "test_external_api_keys_dual_cohere";
const OPENAI_MODEL: &str = "openai/text-embedding-3-small";
const OPENAI_VECTOR_SIZE: u64 = 1536;
const COHERE_MODEL: &str = "cohere/embed-english-v3.0";
const COHERE_VECTOR_SIZE: u64 = 1024;

fn create_client_with_external_keys(external_api_keys: HashMap<String, String>) -> Qdrant {
    Qdrant::from_url(PROXY_URL)
        .skip_compatibility_check()
        .api_key("1234")
        .external_api_keys(external_api_keys)
        .timeout(30u64)
        .build()
        .expect("Failed to build client")
}

async fn setup_collection(client: &Qdrant, collection_name: &str, vector_size: u64) {
    let _ = client.delete_collection(collection_name).await;

    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(vector_size, Distance::Cosine)),
        )
        .await
        .expect("Failed to create collection");
}

fn cohere_document(text: impl Into<String>, input_type: &'static str) -> Document {
    Document {
        text: text.into(),
        model: COHERE_MODEL.to_string(),
        options: HashMap::from([("input_type".to_string(), input_type.into())]),
    }
}

#[tokio::test]
async fn test_upsert_with_external_api_keys() {
    let Some(openai_api_key) = std::env::var("OPENAI_API_KEY").ok() else {
        eprintln!("Skipping test_upsert_with_external_api_keys: OPENAI_API_KEY is not set");
        return;
    };
    let collection_name = UPSERT_COLLECTION_NAME;
    let client = create_client_with_external_keys(HashMap::from([(
        "openai-api-key".to_string(),
        openai_api_key,
    )]));
    setup_collection(&client, collection_name, OPENAI_VECTOR_SIZE).await;

    let doc = Document::new("Qdrant is a vector search engine", OPENAI_MODEL);

    let result = client
        .upsert_points(
            UpsertPointsBuilder::new(
                collection_name,
                vec![PointStruct::new(
                    1,
                    doc,
                    Payload::try_from(json!({"source": "test"})).unwrap(),
                )],
            )
            .wait(true),
        )
        .await;

    assert!(
        result.is_ok(),
        "Upsert with external API keys failed: {result:?}"
    );

    let _ = client.delete_collection(collection_name).await;
}

#[tokio::test]
async fn test_query_with_external_api_keys() {
    let Some(openai_api_key) = std::env::var("OPENAI_API_KEY").ok() else {
        eprintln!("Skipping test_query_with_external_api_keys: OPENAI_API_KEY is not set");
        return;
    };
    let collection_name = QUERY_COLLECTION_NAME;
    let client = create_client_with_external_keys(HashMap::from([(
        "openai-api-key".to_string(),
        openai_api_key,
    )]));
    setup_collection(&client, collection_name, OPENAI_VECTOR_SIZE).await;

    // Upsert a point first
    let doc = Document::new("Qdrant is a vector search engine", OPENAI_MODEL);
    client
        .upsert_points(
            UpsertPointsBuilder::new(
                collection_name,
                vec![PointStruct::new(
                    1,
                    doc,
                    Payload::try_from(json!({"source": "test"})).unwrap(),
                )],
            )
            .wait(true),
        )
        .await
        .expect("Upsert failed");

    // Query with a document (server-side inference)
    let query_doc = Document::new("vector database", OPENAI_MODEL);

    let result = client
        .query(
            QueryPointsBuilder::new(collection_name)
                .query(Query::new_nearest(query_doc))
                .limit(1)
                .with_payload(true),
        )
        .await;

    assert!(
        result.is_ok(),
        "Query with external API keys failed: {result:?}"
    );

    let response = result.unwrap();
    assert_eq!(response.result.len(), 1);
    assert!(response.result[0].payload.contains_key("source"));

    let _ = client.delete_collection(collection_name).await;
}

#[tokio::test]
async fn test_query_with_two_external_api_providers() {
    let Some(openai_api_key) = std::env::var("OPENAI_API_KEY").ok() else {
        eprintln!("Skipping test_query_with_two_external_api_providers: OPENAI_API_KEY is not set");
        return;
    };
    let Some(cohere_api_key) = std::env::var("COHERE_API_KEY").ok() else {
        eprintln!("Skipping test_query_with_two_external_api_providers: COHERE_API_KEY is not set");
        return;
    };

    let client = create_client_with_external_keys(HashMap::from([
        ("openai-api-key".to_string(), openai_api_key),
        ("cohere-api-key".to_string(), cohere_api_key),
    ]));

    setup_collection(&client, DUAL_OPENAI_COLLECTION_NAME, OPENAI_VECTOR_SIZE).await;
    setup_collection(&client, DUAL_COHERE_COLLECTION_NAME, COHERE_VECTOR_SIZE).await;

    let openai_doc = Document::new("OpenAI provider document", OPENAI_MODEL);
    let cohere_doc = cohere_document("Cohere provider document", "search_document");

    let openai_upsert = client
        .upsert_points(
            UpsertPointsBuilder::new(
                DUAL_OPENAI_COLLECTION_NAME,
                vec![PointStruct::new(
                    1,
                    openai_doc,
                    Payload::try_from(json!({"provider": "openai"})).unwrap(),
                )],
            )
            .wait(true),
        )
        .await;
    assert!(
        openai_upsert.is_ok(),
        "OpenAI upsert with external API keys failed: {openai_upsert:?}"
    );

    let cohere_upsert = client
        .upsert_points(
            UpsertPointsBuilder::new(
                DUAL_COHERE_COLLECTION_NAME,
                vec![PointStruct::new(
                    1,
                    cohere_doc,
                    Payload::try_from(json!({"provider": "cohere"})).unwrap(),
                )],
            )
            .wait(true),
        )
        .await;
    assert!(
        cohere_upsert.is_ok(),
        "Cohere upsert with external API keys failed: {cohere_upsert:?}"
    );

    let openai_query = client
        .query(
            QueryPointsBuilder::new(DUAL_OPENAI_COLLECTION_NAME)
                .query(Query::new_nearest(Document::new(
                    "OpenAI provider query",
                    OPENAI_MODEL,
                )))
                .limit(1)
                .with_payload(true),
        )
        .await;
    assert!(
        openai_query.is_ok(),
        "OpenAI query with external API keys failed: {openai_query:?}"
    );

    let cohere_query = client
        .query(
            QueryPointsBuilder::new(DUAL_COHERE_COLLECTION_NAME)
                .query(Query::new_nearest(cohere_document(
                    "Cohere provider query",
                    "search_query",
                )))
                .limit(1)
                .with_payload(true),
        )
        .await;
    assert!(
        cohere_query.is_ok(),
        "Cohere query with external API keys failed: {cohere_query:?}"
    );

    let openai_response = openai_query.unwrap();
    assert_eq!(openai_response.result.len(), 1);
    assert_eq!(
        openai_response.result[0].payload["provider"],
        "openai".into()
    );

    let cohere_response = cohere_query.unwrap();
    assert_eq!(cohere_response.result.len(), 1);
    assert_eq!(
        cohere_response.result[0].payload["provider"],
        "cohere".into()
    );

    let _ = client.delete_collection(DUAL_OPENAI_COLLECTION_NAME).await;
    let _ = client.delete_collection(DUAL_COHERE_COLLECTION_NAME).await;
}
