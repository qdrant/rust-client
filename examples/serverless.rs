//! Example of using the Qdrant Serverless client.
//!
//! **In development — do not use yet.** This client is experimental and unstable;
//! it may change without notice and is not ready for production or general use.
//!
//! Collection management uses the simplified serverless API; point operations
//! (query, upsert, ...) work like in the regular client (unsupported cluster
//! parameters are stripped before each RPC).
//!
//! The example covers:
//! - a full-text index with BM25 scoring, queried with [`Query::new_text`]
//! - the `Code` tokenizer for source code snippets
//! - nested payload indexes over the objects of an array (`offers[].color`)
//! - collection stats from `get_collection` / `list_collections`
//!
//! ```bash
//! cargo run --example serverless --features serde
//! ```

use qdrant_client::serverless::{
    CollectionConfig, CollectionStatus, Condition, DenseVectorConfig, Distance, Filter, FloatIndex,
    KeywordIndex, ListCollectionsBuilder, Payload, PointStruct, QdrantError, QdrantServerless,
    Query, QueryPointsBuilder, Range, TextIndex, TextQueryBuilder, Tokenizer, UpdatesStatus,
    UpsertPointsBuilder,
};

#[tokio::main]
async fn main() -> Result<(), QdrantError> {
    let client =
        QdrantServerless::from_url("https://serverless.plush-volt.aws.development-cloud.qdrant.io")
            .api_key("<your api key>")
            .build()?;

    let collection_name = "my-collection";

    // make the example rerunnable: creating an existing collection returns ALREADY_EXISTS
    if client.collection_exists(collection_name).await? {
        client.delete_collection(collection_name).await?;
    }

    // serverless-specific collection management: no quantization, wal,
    // segment number etc. - the serverless manager decides those
    let result = client
        .create_collection(
            collection_name,
            CollectionConfig::new()
                .dense_vector(DenseVectorConfig::new(4, Distance::Cosine))
                .payload_index("color", KeywordIndex::new())
                // full-text index that can rank points by BM25, see the text query below
                .payload_index(
                    "description",
                    TextIndex::new()
                        .tokenizer(Tokenizer::Word)
                        .lowercase(true)
                        .bm25_scoring(),
                )
                // source code: splits `parseHttpRequest` / `parse_http_request` into subwords
                .payload_index(
                    "snippet",
                    TextIndex::new().tokenizer(Tokenizer::Code).bm25_scoring(),
                )
                // nested indexes: every object of `offers` is indexed as a separate entry,
                // so a nested filter matches color and price on the same offer
                .payload_index("offers[].color", KeywordIndex::new().nested("offers"))
                .payload_index("offers[].price", FloatIndex::new().nested("offers")),
        )
        .await?;
    println!("create_collection: {result:?}");

    let points = vec![
        PointStruct::new(
            1,
            vec![0.1, 0.2, 0.3, 0.4],
            Payload::try_from(serde_json::json!({
                "color": "red",
                "description": "A vector database written in Rust",
                "snippet": "fn parseHttpRequest(raw: &[u8]) -> HttpRequest",
                "offers": [
                    {"color": "red", "price": 10.0},
                    {"color": "blue", "price": 25.0},
                ],
            }))
            .unwrap(),
        ),
        PointStruct::new(
            2,
            vec![0.4, 0.3, 0.2, 0.1],
            Payload::try_from(serde_json::json!({
                "color": "blue",
                "description": "Full-text search with BM25 ranking",
                "snippet": "def build_search_index(documents): ...",
                "offers": [
                    {"color": "red", "price": 30.0},
                    {"color": "blue", "price": 5.0},
                ],
            }))
            .unwrap(),
        ),
    ];
    client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points).wait(true))
        .await?;

    // collection stats; `None` until the server has computed them
    let info = client.get_collection(collection_name).await?;
    println!(
        "info: status={:?} points={:?} size_bytes={:?} indexed_vectors={:?} updates={:?}",
        info.status, info.point_count, info.size_bytes, info.indexed_vectors_count, info.updates,
    );
    if info.status == Some(CollectionStatus::Yellow) {
        println!("optimization is pending or running; search still works");
    }
    if info.updates.as_ref().map(|u| u.status) == Some(UpdatesStatus::Pending) {
        println!("some acknowledged writes are not yet visible to search");
    }

    for collection in client
        .list_collections(ListCollectionsBuilder::new())
        .await?
        .collections
    {
        println!(
            "collection {}: points={:?} size_bytes={:?}",
            collection.collection_name, collection.point_count, collection.size_bytes,
        );
    }

    // dense vector search
    let hits = client
        .query(
            QueryPointsBuilder::new(collection_name)
                .query(vec![0.1, 0.2, 0.3, 0.4])
                .limit(10),
        )
        .await?;
    println!("vector query: {hits:?}");

    // BM25 ranking over the text index of `description`, selected with `using`
    let hits = client
        .query(
            QueryPointsBuilder::new(collection_name)
                .query(Query::new_text("rust vector database"))
                .using("description")
                .limit(10),
        )
        .await?;
    println!("bm25 query: {hits:?}");

    // BM25 with tuned parameters over code: `http request` matches `parseHttpRequest`
    let hits = client
        .query(
            QueryPointsBuilder::new(collection_name)
                .query(Query::new_text(
                    TextQueryBuilder::new("http request").k(1.5).b(0.5),
                ))
                .using("snippet")
                .limit(10),
        )
        .await?;
    println!("code query: {hits:?}");

    // nested filter: a red offer cheaper than 20 (matches point 1 only)
    let hits = client
        .query(
            QueryPointsBuilder::new(collection_name)
                .query(vec![0.1, 0.2, 0.3, 0.4])
                .filter(Filter::must([Condition::nested(
                    "offers",
                    Filter::must([
                        Condition::matches("color", "red".to_string()),
                        Condition::range(
                            "price",
                            Range {
                                lt: Some(20.0),
                                ..Default::default()
                            },
                        ),
                    ]),
                )]))
                .limit(10),
        )
        .await?;
    println!("nested filter query: {hits:?}");

    client.delete_collection(collection_name).await?;
    Ok(())
}
