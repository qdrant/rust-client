//! The Qdrant Vector Database client
//!
//! This library uses GRPC to connect to the Qdrant server and allows you to
//! access most if not all features. If you find a missing feature, please open
//! an [issue](https://github.com/qdrant/rust-client/issues/new).
//!
//! To work with a Qdrant server, you'll first need to connect by creating a [`Qdrant`] client:
//! ```
//! use qdrant_client::{Qdrant, QdrantConfig, Result};
//!
//!# fn establish_connection(url: &str) -> Result<Qdrant> {
//! let mut config = QdrantConfig::from_url(url);
//! config.api_key = std::env::var("QDRANT_API_KEY").ok();
//! Qdrant::new(Some(config))
//!# }
//! ```
//!
//! Qdrant works with *Collections* of *Points*. To add vector data, you first
//! create a collection:
//!
//! ```
//!# use qdrant_client::{Qdrant, Result};
//!# use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParams, VectorParamsBuilder, VectorsConfig};
//!# use qdrant_client::qdrant::vectors_config::Config;
//!# async fn create_collection(qdrant_client: &Qdrant)
//!# -> Result<()> {
//! let client = Qdrant::new(None).unwrap();
//! let response = client
//!     .create_collection(
//!         CreateCollectionBuilder::new("my_collection")
//!             .vectors_config(VectorParamsBuilder::new(512, Distance::Cosine)),
//!     )
//!     .await?;
//!# Ok(())
//!# }
//! ```
//! The most interesting parts are the two arguments of `VectorParamsBuilder::new`.
//! The first one (`512`) is the length of vectors to store and the second one (`Distance::Cosine`)
//! is the Distance, which is the [`Distance`](qdrant::Distance) measure to gauge
//! similarity for the nearest neighbors search.
//!
//! Now we have a collection, we can insert (or rather upsert) points.
//! Points have an id, one or more vectors and a payload.
//! We can usually do that in bulk, but for this example, we'll add a
//! single point:
//! ```
//!# use qdrant_client::{Qdrant, Result};
//! use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
//!# async fn do_upsert(qdrant_client: &Qdrant)
//!# -> Result<()> {
//! let point = PointStruct::new(
//!     42, // The unique ID of our point
//!     vec![0.0_f32; 512], // The vector
//!     // Our payload
//!     [
//!         ("great", true.into()),
//!         ("level", 9000.into()),
//!         ("text", "Hi Qdrant!".into()),
//!         ("list", vec![1.234f32, 0.815].into()),
//!     ],
//! );
//!
//! let response = qdrant_client
//!     .upsert_points(UpsertPointsBuilder::new("my_collection", vec![point]))
//!     .await?;
//!# Ok(())
//!# }
//! ```
//!
//! Finally, we can retrieve points in various ways, the canonical one being
//! a plain similarity search:
//! ```
//!# use qdrant_client::{Qdrant, Result};
//!# use qdrant_client::qdrant::SearchPointsBuilder;
//!# async fn search(qdrant_client: &Qdrant)
//!# -> Result<()> {
//! let search_request =
//!     SearchPointsBuilder::new("my_collection", vec![0.0_f32; 512], 4).with_payload(true);
//! let response = qdrant_client.search_points(search_request).await?;
//!# Ok(())
//!# }
//! ```
//! The parameter for `SearchPointsBuilder::new()` contsructor are pretty straightforward:
//! Name of the collection, the vector and how many top-k results to return.
//! The `with_payload(true)` call tells qdrant to also return the (full) payload data for each point.
//! You can also add a `.filter()` call to the
//! [`SearchPointsBuilder`](qdrant::SearchPointsBuilder) to filter the result.
//! See the [`Filter`](qdrant::Filter) documentation for details.

// Public modules
pub mod auth;
pub mod builder_types;
pub mod payload;

// Qdrant API types
/// API types
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod qdrant;

// Internal modules
mod builder_ext;
mod channel_pool;
mod filters;
mod grpc_conversions;
mod grpc_macros;
mod manual_builder;
mod qdrant_client;

// Deprecated modules
/// Deprecated Qdrant client
#[deprecated(
    since = "1.10.0",
    note = "use new client at `qdrant_client::Qdrant` instead"
)]
pub mod client;
/// Deprecated Qdrant client config
#[deprecated(
    since = "1.10.0",
    note = "use new config at `qdrant_client::QdrantConfig` instead"
)]
pub mod config;
/// Deprecated error type
#[deprecated(
    since = "1.10.0",
    note = "use new error type at `qdrant_client::Error` instead"
)]
pub mod error;
/// Deprecated prelude
#[deprecated(since = "1.10.0", note = "use types directly")]
pub mod prelude;
/// Deprecated serde helper
#[cfg(feature = "serde")]
#[deprecated(since = "1.10.0", note = "use `Payload::from_json_object` instead")]
pub mod serde;

// Re-exports
pub use crate::qdrant_client::{config::QdrantConfig, error::Error, Qdrant, QdrantBuilder, Result};

// Vendored re-exports
#[doc(no_inline)]
pub use prost_types::Timestamp;

#[cfg(test)]
mod tests {
    use crate::payload::Payload;
    use crate::qdrant::value::Kind::*;
    use crate::qdrant::{
        Condition, CreateCollectionBuilder, CreateFieldIndexCollection, DeletePayloadPointsBuilder,
        DeletePointsBuilder, Distance, FieldType, Filter, GetPointsBuilder, ListValue, PointStruct,
        SearchPointsBuilder, SetPayloadPointsBuilder, SnapshotDownloadBuilder, Struct,
        UpsertPointsBuilder, Value, VectorParamsBuilder,
    };
    use crate::{Qdrant, QdrantConfig};
    use std::collections::HashMap;

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
        let config = QdrantConfig::from_url("http://localhost:6334");
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
