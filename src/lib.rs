//! The [Qdrant](https://qdrant.tech/) - High-Performance Vector Search at Scale - client for Rust.
//!
//! This crate connects to your Qdrant server over gRPC and provides an easy to use API interface
//! for it.
//!
//! # Connect
//!
//! First you'll need to [connect](Qdrant#connect) by creating a [`Qdrant`] client:
//!
//! ```no_run
//! use qdrant_client::Qdrant;
//!# use qdrant_client::QdrantError;
//!
//!# fn establish_connection(url: &str) -> Result<Qdrant, QdrantError> {
//! let client = Qdrant::from_url("http://localhost:6334")
//!     .api_key(std::env::var("QDRANT_API_KEY"))
//!     .build()?;
//!# Ok(client)
//!# }
//! ```
//!
//! # Create collection
//!
//! Qdrant works with [Collections ⧉ ](https://qdrant.tech/documentation/concepts/collections/) of
//! [Points ⧉ ](https://qdrant.tech/documentation/concepts/points/). To add vector data, you first
//! [create a collection](Qdrant::create_collection):
//!
//! ```no_run
//!# use qdrant_client::{Qdrant, QdrantError};
//! use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};
//!
//!# async fn create_collection(client: &Qdrant)
//!# -> Result<(), QdrantError> {
//! let response = client
//!     .create_collection(
//!         CreateCollectionBuilder::new("my_collection")
//!             .vectors_config(VectorParamsBuilder::new(512, Distance::Cosine)),
//!     )
//!     .await?;
//!# Ok(())
//!# }
//! ```
//!
//! The most interesting parts are the two arguments of
//! [`VectorParamsBuilder::new`](qdrant::VectorParamsBuilder::new). The first one (`512`) is the
//! length of vectors to store and the second one ([`Distance::Cosine`](qdrant::Distance::Cosine))
//! is the Distance, which is the [`Distance`](qdrant::Distance) measure to gauge similarity for
//! the nearest neighbors search.
//!
//! Documentation: <https://qdrant.tech/documentation/concepts/collections/#create-a-collection>
//!
//! # Upsert points
//!
//! Now we have a collection, we can insert (or rather upsert) points.
//! Points have an id, one or more vectors and a payload.
//! We can usually do that in bulk, but for this example, we'll add a
//! single point:
//!
//! ```no_run
//!# use qdrant_client::{Qdrant, QdrantError};
//! use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
//!
//!# async fn do_upsert(client: &Qdrant)
//!# -> Result<(), QdrantError> {
//! let points = vec![
//!     PointStruct::new(
//!         42,                 // Uniqe piont ID
//!         vec![0.0_f32; 512], // Vector to upsert
//!         // Attached payload
//!         [
//!             ("great", true.into()),
//!             ("level", 9000.into()),
//!             ("text", "Hi Qdrant!".into()),
//!             ("list", vec![1.234f32, 0.815].into()),
//!         ],
//!     ),
//! ];
//!
//! let response = client
//!     .upsert_points(UpsertPointsBuilder::new("my_collection", points))
//!     .await?;
//!# Ok(())
//!# }
//! ```
//!
//! Documentation: <https://qdrant.tech/documentation/concepts/points/#upload-points>
//!
//! # Search
//!
//! Finally, we can retrieve points in various ways, the common one being a plain similarity
//! search:
//!
//! ```no_run
//!# use qdrant_client::{Qdrant, QdrantError};
//! use qdrant_client::qdrant::SearchPointsBuilder;
//!
//!# async fn search(client: &Qdrant)
//!# -> Result<(), QdrantError> {
//! let search_request = SearchPointsBuilder::new(
//!     "my_collection",    // Collection name
//!     vec![0.0_f32; 512], // Cearch vector
//!     4,                  // Search limit, number of results to return
//! ).with_payload(true);
//!
//! let response = client.search_points(search_request).await?;
//!# Ok(())
//!# }
//! ```
//!
//! The parameter for [`SearchPointsBuilder::new()`](qdrant::SearchPointsBuilder::new) contsructor
//! are pretty straightforward: name of the collection, the vector and how many top-k results to
//! return. The [`with_payload(true)`](qdrant::SearchPointsBuilder::with_payload) call tells qdrant
//! to also return the (full) payload data for each point. You can also add a
//! [`filter()`](qdrant::SearchPointsBuilder::filter) call to the
//! [`SearchPointsBuilder`](qdrant::SearchPointsBuilder) to filter the result. See the
//! [`Filter`](qdrant::Filter) documentation for details.
//!
//! Documentation: <https://qdrant.tech/documentation/concepts/search/>

#![doc(html_logo_url = "https://qdrant.tech/favicon/android-chrome-192x192.png")]
#![doc(issue_tracker_base_url = "https://github.com/qdrant/rust-client/issues/")]

// Generated Qdrant API types
/// API types
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod qdrant;

// Internal modules
mod auth;
mod builder_ext;
mod builder_types;
mod channel_pool;
mod filters;
mod grpc_conversions;
mod grpc_macros;
mod manual_builder;
mod payload;
mod qdrant_client;

// Deprecated modules
/// Deprecated Qdrant client
#[deprecated(
    since = "1.10.0",
    note = "use new client at `qdrant_client::Qdrant` instead"
)]
#[doc(hidden)]
pub mod client;
/// Deprecated error type
#[deprecated(
    since = "1.10.0",
    note = "use new error type at `qdrant_client::Error` instead"
)]
#[doc(hidden)]
pub mod error;
/// Deprecated prelude
#[deprecated(since = "1.10.0", note = "use types directly")]
#[doc(hidden)]
pub mod prelude;
/// Deprecated serde helper
#[cfg(feature = "serde")]
#[deprecated(since = "1.10.0", note = "use `Payload::try_from` instead")]
#[doc(hidden)]
pub mod serde;

// Re-exports
pub use crate::payload::Payload;
pub use crate::qdrant_client::error::QdrantError;
pub use crate::qdrant_client::{Qdrant, QdrantBuilder};

/// Client configuration
pub mod config {
    pub use crate::qdrant_client::config::{
        AsOptionApiKey, AsTimeout, CompressionEncoding, QdrantConfig,
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::payload::Payload;
    use crate::qdrant::value::Kind::*;
    use crate::qdrant::{
        Condition, CreateCollectionBuilder, CreateFieldIndexCollection, DeletePayloadPointsBuilder,
        DeletePointsBuilder, Distance, FieldType, Filter, GetPointsBuilder, ListValue, PointStruct,
        SearchPointsBuilder, SetPayloadPointsBuilder, SnapshotDownloadBuilder, Struct,
        UpsertPointsBuilder, Value, VectorParamsBuilder,
    };
    use crate::Qdrant;

    #[test]
    fn display() {
        let value = Value {
            kind: Some(StructValue(Struct {
                fields: [
                    ("text", StringValue("Hi Qdrant!".into())),
                    ("int", IntegerValue(42)),
                    ("float", DoubleValue(1.23)),
                    (
                        "list",
                        ListValue(ListValue {
                            values: vec![Value {
                                kind: Some(NullValue(0)),
                            }],
                        }),
                    ),
                    (
                        "struct",
                        StructValue(Struct {
                            fields: [(
                                "bool".into(),
                                Value {
                                    kind: Some(BoolValue(true)),
                                },
                            )]
                            .into(),
                        }),
                    ),
                ]
                .into_iter()
                .map(|(k, v)| (k.into(), Value { kind: Some(v) }))
                .collect(),
            })),
        };
        let text = format!("{}", value);
        assert!([
            "\"float\":1.23",
            "\"list\":[null]",
            "\"struct\":{\"bool\":true}",
            "\"int\":42",
            "\"text\":\"Hi Qdrant!\""
        ]
        .into_iter()
        .all(|item| text.contains(item)));
    }

    #[tokio::test]
    async fn test_qdrant_queries() -> anyhow::Result<()> {
        let config = Qdrant::from_url("http://localhost:6334");
        let client = Qdrant::new(Some(config))?;

        let health = client.health_check().await?;
        println!("{:?}", health);

        let collections_list = client.list_collections().await?;
        println!("{:?}", collections_list);

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
        println!("{:#?}", collection_info);

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

        let mut search_points =
            SearchPointsBuilder::new(collection_name, vec![11.; 10], 10).build();

        // Keyword filter result
        search_points.filter = Some(Filter::all([Condition::matches("foo", "Bar".to_string())]));
        let search_result = client.search_points(search_points.clone()).await?;
        println!("{:#?}", search_result);

        assert!(!search_result.result.is_empty());

        // Existing implementations full text search filter result (`Condition::matches`)
        search_points.filter = Some(Filter::all([Condition::matches(
            "sub_payload.foo",
            "Not ".to_string(),
        )]));
        let search_result = client.search_points(search_points.clone()).await?;
        assert!(!search_result.result.is_empty());

        // Full text search filter result (`Condition::matches_text`)
        search_points.filter = Some(Filter::all([Condition::matches_text(
            "sub_payload.foo",
            "Not",
        )]));
        let search_result = client.search_points(search_points).await?;
        assert!(!search_result.result.is_empty());

        eprintln!("search_result = {:#?}", search_result);

        // Override payload of the existing point
        let new_payload: Payload = vec![("foo", "BAZ".into())]
            .into_iter()
            .collect::<HashMap<_, Value>>()
            .into();

        client
            .set_payload(
                SetPayloadPointsBuilder::new(collection_name, new_payload).points_selector([0]),
            )
            .await?;

        // Delete some payload fields
        client
            .delete_payload(
                DeletePayloadPointsBuilder::new(collection_name, ["sub_payload".into()])
                    .points_selector([0]),
            )
            .await?;

        let points = client
            .get_points(
                GetPointsBuilder::new(collection_name, [0.into()])
                    .with_vectors(true)
                    .with_payload(true),
            )
            .await?;

        assert_eq!(points.result.len(), 1);
        let point = points.result[0].clone();
        assert!(point.payload.contains_key("foo"));
        assert!(!point.payload.contains_key("sub_payload"));

        client
            .delete_points(
                DeletePointsBuilder::new(collection_name)
                    .points([0])
                    .wait(true),
            )
            .await?;

        // Access raw point api with client
        client
            .with_points_client(|mut client| async move {
                client
                    .create_field_index(CreateFieldIndexCollection {
                        collection_name: collection_name.to_string(),
                        wait: None,
                        field_name: "foo".to_string(),
                        field_type: Some(FieldType::Keyword as i32),
                        field_index_params: None,
                        ordering: None,
                    })
                    .await
            })
            .await?;

        client.create_snapshot(collection_name).await?;

        #[cfg(feature = "download_snapshots")]
        client
            .download_snapshot(SnapshotDownloadBuilder::new("test.tar", collection_name))
            .await?;

        Ok(())
    }
}
