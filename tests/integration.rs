//! Integration tests using testcontainers
//!
//! These tests spin up a Qdrant container and run the full test suite against it.
//! The container is shared across all tests for efficiency.

mod test_utils;

use std::collections::HashMap;

#[cfg(feature = "download_snapshots")]
use qdrant_client::qdrant::SnapshotDownloadBuilder;
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, DeletePayloadPointsBuilder, DeletePointsBuilder, Distance,
    Filter, GetPointsBuilder, PointStruct, QueryPointsBuilder, SearchPointsBuilder,
    SetPayloadPointsBuilder, UpsertPointsBuilder, Value, VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant};
use test_utils::get_or_create_container;

#[tokio::test]
async fn test_qdrant_queries() -> anyhow::Result<()> {
    let container = get_or_create_container().await;

    let client = Qdrant::from_url(&container.grpc_url)
        .timeout(10u64) // larger timeout to account for the slow snapshot creation
        .build()?;

    let health = client.health_check().await?;
    println!("{health:?}");

    let collections_list = client.list_collections().await?;
    println!("{collections_list:?}");

    let collection_name = "test_qdrant_queries";
    client.delete_collection(collection_name).await?;

    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(10, Distance::Cosine)),
        )
        .await?;

    let exists = client.collection_exists(collection_name).await?;
    assert!(exists);

    let collection_info = client.collection_info(collection_name).await?;
    println!("{collection_info:#?}");

    let mut sub_payload = Payload::new();
    sub_payload.insert("foo", "Not bar");

    let payload: Payload = vec![
        ("foo", "Bar".into()),
        ("bar", 12.into()),
        ("sub_payload", sub_payload.into()),
    ]
    .into_iter()
    .collect::<HashMap<_, Value>>()
    .into();

    let points = vec![PointStruct::new(0, vec![12.; 10], payload)];
    client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points).wait(true))
        .await?;

    let mut search_points = SearchPointsBuilder::new(collection_name, vec![11.; 10], 10).build();

    // Keyword filter result
    search_points.filter = Some(Filter::all([Condition::matches("foo", "Bar".to_string())]));
    let search_result = client.search_points(search_points.clone()).await?;
    eprintln!("search_result = {search_result:#?}");
    assert!(!search_result.result.is_empty());

    // Existing implementations full text search filter result (`Condition::matches`)
    search_points.filter = Some(Filter::all([Condition::matches(
        "sub_payload.foo",
        "Not ".to_string(),
    )]));
    let search_result = client.search_points(search_points.clone()).await?;
    eprintln!("search_result = {search_result:#?}");
    assert!(!search_result.result.is_empty());

    // Full text search filter result (`Condition::matches_text`)
    search_points.filter = Some(Filter::all([Condition::matches_text(
        "sub_payload.foo",
        "Not",
    )]));
    let search_result = client.search_points(search_points).await?;
    eprintln!("search_result = {search_result:#?}");
    assert!(!search_result.result.is_empty());

    // Override payload of the existing point
    let new_payload: Payload = vec![("foo", "BAZ".into())]
        .into_iter()
        .collect::<HashMap<_, Value>>()
        .into();

    let payload_result = client
        .set_payload(
            SetPayloadPointsBuilder::new(collection_name, new_payload).points_selector([0]),
        )
        .await?;
    eprintln!("payload_result = {payload_result:#?}");

    // Delete some payload fields
    client
        .delete_payload(
            DeletePayloadPointsBuilder::new(collection_name, ["sub_payload".into()])
                .points_selector([0]),
        )
        .await?;

    let get_points_result = client
        .get_points(
            GetPointsBuilder::new(collection_name, [0.into()])
                .with_vectors(true)
                .with_payload(true),
        )
        .await?;
    eprintln!("get_points_result = {get_points_result:#?}");
    assert_eq!(get_points_result.result.len(), 1);
    let point = get_points_result.result[0].clone();
    assert!(point.payload.contains_key("foo"));
    assert!(!point.payload.contains_key("sub_payload"));

    let delete_points_result = client
        .delete_points(
            DeletePointsBuilder::new(collection_name)
                .points([0])
                .wait(true),
        )
        .await?;
    eprintln!("delete_points_result = {delete_points_result:#?}");

    // slow operation
    let snapshot_result = client.create_snapshot(collection_name).await?;
    eprintln!("snapshot_result = {snapshot_result:#?}");

    #[cfg(feature = "download_snapshots")]
    client
        .download_snapshot(SnapshotDownloadBuilder::new("test.tar", collection_name))
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_create_collection_and_do_the_search() -> anyhow::Result<()> {
    let container = get_or_create_container().await;

    let client = Qdrant::from_url(&container.grpc_url).build()?;

    let health = client.health_check().await?;
    println!("{health:?}");

    let collection_name = "test_create_collection_and_do_the_search";

    client.delete_collection(collection_name).await?;

    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(10, Distance::Cosine)),
        )
        .await?;

    let points = vec![PointStruct::new(
        0,
        vec![12.; 10],
        Payload::try_from(serde_json::json!({
            "field": "value"
        }))
        .unwrap(),
    )];

    client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points).wait(true))
        .await?;

    let search_points = SearchPointsBuilder::new(collection_name, vec![11.; 10], 10).build();

    let search_result = client.search_points(search_points).await?;
    eprintln!("search_result = {search_result:#?}");
    assert!(!search_result.result.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_query() {
    let container = get_or_create_container().await;

    let client = Qdrant::from_url(&container.grpc_url).build().unwrap();

    let collection_name = "test_query";

    client.delete_collection(collection_name).await.ok();

    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(10, Distance::Cosine)),
        )
        .await
        .unwrap();

    let points = vec![PointStruct::new(
        0,
        vec![12.; 10],
        Payload::try_from(serde_json::json!({
            "field": "value"
        }))
        .unwrap(),
    )];

    client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points).wait(true))
        .await
        .unwrap();

    let query_result = client
        .query(QueryPointsBuilder::new(collection_name).query(vec![11.; 10]))
        .await
        .unwrap();
    eprintln!("query_result = {query_result:#?}");
    assert!(!query_result.result.is_empty());
}
