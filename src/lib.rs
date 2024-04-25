//! The Qdrant Vector Database client
//!
//! This library uses GRPC to connect to the Qdrant server and allows you to
//! access most if not all features. If you find a missing feature, please open
//! an [issue](https://github.com/qdrant/rust-client/issues/new).
//!
//! If you use this library, you'll likely want to import the usual types and
//! functions:
//! ```
//!#[allow(unused_import)]
//! use qdrant_client::prelude::*;
//! ```
//!
//! To work with a Qdrant database, you'll first need to connect by creating a
//! [`QdrantClient`](crate::client::QdrantClient):
//! ```
//!# use qdrant_client::prelude::*;
//!# fn establish_connection(url: &str) -> anyhow::Result<QdrantClient> {
//! let mut config = QdrantClientConfig::from_url(url);
//! config.api_key = std::env::var("QDRANT_API_KEY").ok();
//! QdrantClient::new(Some(config))
//!# }
//! ```
//!
//! Qdrant works with *Collections* of *Points*. To add vector data, you first
//! create a collection:
//!
//! ```
//!# use qdrant_client::prelude::*;
//! use qdrant_client::qdrant::{VectorParams, VectorsConfig};
//! use qdrant_client::qdrant::vectors_config::Config;
//!# async fn create_collection(qdrant_client: &QdrantClient)
//!# -> Result<(), Box<dyn std::error::Error>> {
//! let response = qdrant_client
//!     .create_collection(&CreateCollection {
//!         collection_name: "my_collection".into(),
//!         vectors_config: Some(VectorsConfig {
//!             config: Some(Config::Params(VectorParams {
//!                 size: 512,
//!                 distance: Distance::Cosine as i32,
//!                 ..Default::default()
//!             })),
//!         }),
//!         ..Default::default()
//!     })
//!     .await?;
//!# Ok(())
//!# }
//! ```
//! The most interesting parts are the `collection_name` and the
//! `vectors_config.size` (the length of vectors to store) and `distance`
//! (which is the [`Distance`](crate::qdrant::Distance) measure to gauge
//! similarity for the nearest neighbors search).
//!
//! Now we have a collection, we can insert (or rather upsert) points.
//! Points have an id, one or more vectors and a payload.
//! We can usually do that in bulk, but for this example, we'll add a
//! single point:
//! ```
//!# use qdrant_client::{prelude::*, qdrant::PointId};
//!# async fn do_upsert(qdrant_client: &QdrantClient)
//!# -> Result<(), Box<dyn std::error::Error>> {
//! let point = PointStruct {
//!     id: Some(PointId::from(42)), // unique u64 or String
//!     vectors: Some(vec![0.0_f32; 512].into()),
//!     payload: std::collections::HashMap::from([
//!         ("great".into(), Value::from(true)),
//!         ("level".into(), Value::from(9000)),
//!         ("text".into(), Value::from("Hi Qdrant!")),
//!         ("list".into(), Value::from(vec![1.234, 0.815])),
//!     ]),
//! };
//!
//! let response = qdrant_client
//!     .upsert_points("my_collection", None, vec![point], None)
//!     .await?;
//!# Ok(())
//!# }
//! ```
//!
//! Finally, we can retrieve points in various ways, the canonical one being
//! a plain similarity search:
//! ```
//!# use qdrant_client::prelude::*;
//!# async fn search(qdrant_client: &QdrantClient)
//!# -> Result<(), Box<dyn std::error::Error>> {
//! let response = qdrant_client
//!     .search_points(&SearchPoints {
//!         collection_name: "my_collection".to_string(),
//!         vector: vec![0.0_f32; 512],
//!         limit: 4,
//!         with_payload: Some(true.into()),
//!         ..Default::default()
//!     })
//!     .await?;
//!# Ok(())
//!# }
//! ```
//!
//! You can also add a `filters: Some(filters)` field to the
//! [`SearchPoints`](crate::qdrant::SearchPoints) argument to filter the
//! result. See the [`Filter`](crate::qdrant::Filter) documentation for
//! details.

mod channel_pool;
pub mod client;
pub mod prelude;
// Do not lint/fmt code that is generated by tonic
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod qdrant;
pub mod auth;
pub mod config;
pub mod error;
pub mod filters;
pub mod grpc_ext;
pub mod payload;
#[cfg(feature = "serde")]
pub mod serde;

use error::NotA;
use qdrant::{value::Kind::*, ListValue, RetrievedPoint, ScoredPoint, Struct, Value};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

#[doc(no_inline)]
pub use prost_types::Timestamp;

static NULL_VALUE: Value = Value {
    kind: Some(NullValue(0)),
};

impl RetrievedPoint {
    /// get a payload value for the specified key. If the key is not present,
    /// this will return a null value.
    ///
    /// # Examples:
    /// ```
    /// use qdrant_client::qdrant::RetrievedPoint;
    /// let point = RetrievedPoint::default();
    /// assert!(point.get("not_present").is_null());
    /// ````
    pub fn get(&self, key: &str) -> &Value {
        self.payload.get(key).unwrap_or(&NULL_VALUE)
    }
}

impl ScoredPoint {
    /// get a payload value for the specified key. If the key is not present,
    /// this will return a null value.
    ///
    /// # Examples:
    /// ```
    /// use qdrant_client::qdrant::ScrollPoints;
    /// let point = ScrollPoints::default();
    /// assert!(point.get("not_present").is_null());
    /// ````
    pub fn get(&self, key: &str) -> &Value {
        self.payload.get(key).unwrap_or(&NULL_VALUE)
    }
}

macro_rules! extract {
    ($kind:ident, $check:ident) => {
        /// check if this value is a
        #[doc = stringify!($kind)]
        pub fn $check(&self) -> bool {
            matches!(self.kind, Some($kind(_)))
        }
    };
    ($kind:ident, $check:ident, $extract:ident, $ty:ty) => {
        extract!($kind, $check);

        /// extract the contents if this value is a
        #[doc = stringify!($kind)]
        pub fn $extract(&self) -> Option<$ty> {
            if let Some($kind(v)) = self.kind {
                Some(v)
            } else {
                None
            }
        }
    };
    ($kind:ident, $check:ident, $extract:ident, ref $ty:ty) => {
        extract!($kind, $check);

        /// extract the contents if this value is a
        #[doc = stringify!($kind)]
        pub fn $extract(&self) -> Option<&$ty> {
            if let Some($kind(v)) = &self.kind {
                Some(v)
            } else {
                None
            }
        }
    };
}

impl Value {
    extract!(NullValue, is_null);
    extract!(BoolValue, is_bool, as_bool, bool);
    extract!(IntegerValue, is_integer, as_integer, i64);
    extract!(DoubleValue, is_double, as_double, f64);
    extract!(StringValue, is_str, as_str, ref String);
    extract!(ListValue, is_list, as_list, ref [Value]);
    extract!(StructValue, is_struct, as_struct, ref Struct);

    #[cfg(feature = "serde")]
    /// convert this into a `serde_json::Value`
    ///
    /// # Examples:
    ///
    /// ```
    /// use serde_json::json;
    /// use qdrant_client::prelude::*;
    /// use qdrant_client::qdrant::{value::Kind::*, Struct};
    /// let value = Value { kind: Some(StructValue(Struct {
    ///     fields: [
    ///         ("text".into(), Value { kind: Some(StringValue("Hi Qdrant!".into())) }),
    ///         ("int".into(), Value { kind: Some(IntegerValue(42))}),
    ///     ].into()
    /// }))};
    /// assert_eq!(value.into_json(), json!({
    ///    "text": "Hi Qdrant!",
    ///    "int": 42
    /// }));
    /// ```
    pub fn into_json(self) -> serde_json::Value {
        use serde_json::Value as JsonValue;
        match self.kind {
            Some(BoolValue(b)) => JsonValue::Bool(b),
            Some(IntegerValue(i)) => JsonValue::from(i),
            Some(DoubleValue(d)) => JsonValue::from(d),
            Some(StringValue(s)) => JsonValue::String(s),
            Some(ListValue(vs)) => vs.into_iter().map(Value::into_json).collect(),
            Some(StructValue(s)) => s
                .fields
                .into_iter()
                .map(|(k, v)| (k, v.into_json()))
                .collect(),
            Some(NullValue(_)) | None => JsonValue::Null,
        }
    }
}

#[cfg(feature = "serde")]
impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        value.into_json()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            Some(BoolValue(b)) => write!(f, "{}", b),
            Some(IntegerValue(i)) => write!(f, "{}", i),
            Some(DoubleValue(v)) => write!(f, "{}", v),
            Some(StringValue(s)) => write!(f, "{:?}", s),
            Some(ListValue(vs)) => {
                let mut i = vs.values.iter();
                write!(f, "[")?;
                if let Some(first) = i.next() {
                    write!(f, "{}", first)?;
                    for v in i {
                        write!(f, ",{}", v)?;
                    }
                }
                write!(f, "]")
            }
            Some(StructValue(s)) => {
                let mut i = s.fields.iter();
                write!(f, "{{")?;
                if let Some((key, value)) = i.next() {
                    write!(f, "{:?}:{}", key, value)?;
                    for (key, value) in i {
                        write!(f, ",{:?}:{}", key, value)?;
                    }
                }
                write!(f, "}}")
            }
            _ => write!(f, "null"),
        }
    }
}

impl Value {
    /// try to get an iterator over the items of the contained list value, if any
    pub fn iter_list(&self) -> Result<impl Iterator<Item = &Value>, NotA<ListValue>> {
        if let Some(ListValue(values)) = &self.kind {
            Ok(values.iter())
        } else {
            Err(NotA::default())
        }
    }

    /// try to get a field from the struct if this value contains one
    pub fn get_struct(&self, key: &str) -> Result<&Value, NotA<Struct>> {
        if let Some(StructValue(Struct { fields })) = &self.kind {
            Ok(fields.get(key).unwrap_or(&NULL_VALUE))
        } else {
            Err(NotA::default())
        }
    }
}

impl std::ops::Deref for ListValue {
    type Target = [Value];

    fn deref(&self) -> &[Value] {
        &self.values
    }
}

impl IntoIterator for ListValue {
    type Item = Value;

    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl ListValue {
    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.values.iter()
    }
}

impl Hash for qdrant::PointId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use qdrant::point_id::PointIdOptions::{Num, Uuid};
        match &self.point_id_options {
            Some(Num(u)) => state.write_u64(*u),
            Some(Uuid(s)) => s.hash(state),
            None => {}
        }
    }
}

impl Hash for qdrant::ScoredPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Hash for qdrant::RetrievedPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::qdrant::value::Kind::*;
    use crate::qdrant::vectors_config::Config;
    use crate::qdrant::{
        Condition, CreateFieldIndexCollection, FieldType, Filter, ListValue, Struct, Value,
        VectorParams, VectorsConfig,
    };
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
        let config = QdrantClientConfig::from_url("http://localhost:6334");
        let client = QdrantClient::new(Some(config))?;

        let health = client.health_check().await?;
        println!("{:?}", health);

        let collections_list = client.list_collections().await?;
        println!("{:?}", collections_list);

        let collection_name = "test";
        client.delete_collection(collection_name).await?;

        client
            .create_collection(&CreateCollection {
                collection_name: collection_name.into(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 10,
                        distance: Distance::Cosine.into(),
                        hnsw_config: None,
                        quantization_config: None,
                        on_disk: None,
                        datatype: None,
                    })),
                }),
                ..Default::default()
            })
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
            .upsert_points_blocking(collection_name, None, points, None)
            .await?;

        let mut search_points = SearchPoints {
            collection_name: collection_name.into(),
            vector: vec![11.; 10],
            limit: 10,
            with_payload: Some(true.into()),
            ..Default::default()
        };

        // Keyword filter result
        search_points.filter = Some(Filter::all([Condition::matches("foo", "Bar".to_string())]));
        let search_result = client.search_points(&search_points).await?;
        assert!(!search_result.result.is_empty());

        // Existing implementations full text search filter result (`Condition::matches`)
        search_points.filter = Some(Filter::all([Condition::matches(
            "sub_payload.foo",
            "Not ".to_string(),
        )]));
        let search_result = client.search_points(&search_points).await?;
        assert!(!search_result.result.is_empty());

        // Full text search filter result (`Condition::matches_text`)
        search_points.filter = Some(Filter::all([Condition::matches_text(
            "sub_payload.foo",
            "Not",
        )]));
        let search_result = client.search_points(&search_points).await?;
        assert!(!search_result.result.is_empty());

        eprintln!("search_result = {:#?}", search_result);

        // Override payload of the existing point
        let new_payload: Payload = vec![("foo", "BAZ".into())]
            .into_iter()
            .collect::<HashMap<_, Value>>()
            .into();
        client
            .set_payload(
                collection_name,
                None,
                &vec![0.into()].into(),
                new_payload,
                None,
                None,
            )
            .await?;

        // Delete some payload fields
        client
            .delete_payload_blocking(
                collection_name,
                None,
                &vec![0.into()].into(),
                vec!["sub_payload".to_string()],
                None,
            )
            .await?;

        // retrieve points
        let points = client
            .get_points(
                collection_name,
                None,
                &[0.into()],
                Some(true),
                Some(true),
                None,
            )
            .await?;

        assert_eq!(points.result.len(), 1);
        let point = points.result[0].clone();
        assert!(point.payload.contains_key("foo"));
        assert!(!point.payload.contains_key("sub_payload"));

        client
            .delete_points(collection_name, None, &vec![0.into()].into(), None)
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
            .download_snapshot("test.tar", collection_name, None, None)
            .await?;

        Ok(())
    }
}
