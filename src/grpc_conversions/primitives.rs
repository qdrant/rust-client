use std::collections::HashMap;

use crate::prelude::point_id::PointIdOptions;
use crate::prelude::{DeleteCollection, Value};
use crate::qdrant::value::Kind;
use crate::qdrant::{
    shard_key, with_payload_selector, with_vectors_selector, CollectionClusterInfoRequest,
    CollectionExistsRequest, CreateSnapshotRequest, DeleteAlias, DeleteCollectionBuilder,
    DeleteFullSnapshotRequest, GetCollectionInfoRequest, IsEmptyCondition, IsNullCondition,
    ListCollectionAliasesRequest, ListSnapshotsRequest, PayloadExcludeSelector,
    PayloadIncludeSelector, PointId, RepeatedIntegers, RepeatedStrings, ShardKey, ShardKeySelector,
    SparseIndices, SparseVectorConfig, SparseVectorParams, Struct, VectorParams, VectorParamsDiff,
    VectorParamsDiffMap, VectorParamsMap, VectorsSelector, WithPayloadSelector,
    WithVectorsSelector,
};

impl From<bool> for WithPayloadSelector {
    fn from(flag: bool) -> Self {
        WithPayloadSelector {
            selector_options: Some(with_payload_selector::SelectorOptions::Enable(flag)),
        }
    }
}

impl From<Vec<&str>> for WithPayloadSelector {
    fn from(fields: Vec<&str>) -> Self {
        WithPayloadSelector {
            selector_options: Some(with_payload_selector::SelectorOptions::Include(
                PayloadIncludeSelector {
                    fields: fields.into_iter().map(|f| f.to_string()).collect(),
                },
            )),
        }
    }
}

impl From<Vec<&str>> for WithVectorsSelector {
    fn from(names: Vec<&str>) -> Self {
        WithVectorsSelector {
            selector_options: Some(with_vectors_selector::SelectorOptions::Include(
                VectorsSelector {
                    names: names.into_iter().map(|name| name.to_string()).collect(),
                },
            )),
        }
    }
}

impl From<bool> for WithVectorsSelector {
    fn from(flag: bool) -> Self {
        WithVectorsSelector {
            selector_options: Some(with_vectors_selector::SelectorOptions::Enable(flag)),
        }
    }
}

impl From<f32> for Value {
    fn from(val: f32) -> Self {
        Self {
            kind: Some(Kind::DoubleValue(val as f64)),
        }
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Self {
            kind: Some(Kind::DoubleValue(val)),
        }
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Self {
            kind: Some(Kind::IntegerValue(val)),
        }
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Self {
            kind: Some(Kind::BoolValue(val)),
        }
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Self {
            kind: Some(Kind::StringValue(val)),
        }
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Self {
            kind: Some(Kind::StringValue(val.into())),
        }
    }
}

impl<T> From<Vec<(&str, T)>> for Value
where
    T: Into<Value>,
{
    fn from(val: Vec<(&str, T)>) -> Self {
        Self {
            kind: Some(Kind::StructValue(Struct {
                fields: val
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.into()))
                    .collect(),
            })),
        }
    }
}

impl From<String> for shard_key::Key {
    fn from(keyword: String) -> Self {
        shard_key::Key::Keyword(keyword)
    }
}

impl From<u64> for shard_key::Key {
    fn from(number: u64) -> Self {
        shard_key::Key::Number(number)
    }
}

impl From<String> for ShardKey {
    fn from(keyword: String) -> Self {
        ShardKey {
            key: Some(shard_key::Key::Keyword(keyword)),
        }
    }
}

impl From<u64> for ShardKey {
    fn from(number: u64) -> Self {
        ShardKey {
            key: Some(shard_key::Key::Number(number)),
        }
    }
}

impl From<String> for ShardKeySelector {
    fn from(keyword: String) -> Self {
        ShardKeySelector {
            shard_keys: vec![ShardKey::from(keyword)],
        }
    }
}

impl From<u64> for ShardKeySelector {
    fn from(number: u64) -> Self {
        ShardKeySelector {
            shard_keys: vec![ShardKey::from(number)],
        }
    }
}

impl From<Vec<String>> for ShardKeySelector {
    fn from(keywords: Vec<String>) -> Self {
        ShardKeySelector {
            shard_keys: keywords.into_iter().map(ShardKey::from).collect(),
        }
    }
}

impl From<Vec<u64>> for ShardKeySelector {
    fn from(numbers: Vec<u64>) -> Self {
        ShardKeySelector {
            shard_keys: numbers.into_iter().map(ShardKey::from).collect(),
        }
    }
}

impl From<String> for PointId {
    fn from(val: String) -> Self {
        Self {
            point_id_options: Some(PointIdOptions::Uuid(val)),
        }
    }
}

impl From<&str> for PointId {
    fn from(val: &str) -> Self {
        Self::from(val.to_string())
    }
}

impl From<u64> for PointId {
    fn from(val: u64) -> Self {
        Self {
            point_id_options: Some(PointIdOptions::Num(val)),
        }
    }
}

impl From<Vec<u32>> for SparseIndices {
    fn from(value: Vec<u32>) -> Self {
        Self { data: value }
    }
}

impl From<HashMap<String, VectorParams>> for VectorParamsMap {
    fn from(value: HashMap<String, VectorParams>) -> Self {
        VectorParamsMap { map: value }
    }
}

impl From<HashMap<String, VectorParamsDiff>> for VectorParamsDiffMap {
    fn from(value: HashMap<String, VectorParamsDiff>) -> Self {
        VectorParamsDiffMap { map: value }
    }
}

impl From<HashMap<String, SparseVectorParams>> for SparseVectorConfig {
    fn from(value: HashMap<String, SparseVectorParams>) -> Self {
        Self { map: value }
    }
}

impl From<Vec<String>> for PayloadIncludeSelector {
    fn from(value: Vec<String>) -> Self {
        Self { fields: value }
    }
}

impl From<Vec<String>> for PayloadExcludeSelector {
    fn from(value: Vec<String>) -> Self {
        Self { fields: value }
    }
}

impl From<Vec<String>> for RepeatedStrings {
    fn from(value: Vec<String>) -> Self {
        Self { strings: value }
    }
}

impl From<Vec<i64>> for RepeatedIntegers {
    fn from(value: Vec<i64>) -> Self {
        Self { integers: value }
    }
}

impl From<HashMap<String, Value>> for Struct {
    fn from(value: HashMap<String, Value>) -> Self {
        Self { fields: value }
    }
}

impl From<Vec<String>> for VectorsSelector {
    fn from(value: Vec<String>) -> Self {
        VectorsSelector { names: value }
    }
}

impl From<String> for IsEmptyCondition {
    fn from(value: String) -> Self {
        Self { key: value }
    }
}

impl From<String> for IsNullCondition {
    fn from(value: String) -> Self {
        Self { key: value }
    }
}

impl From<bool> for with_payload_selector::SelectorOptions {
    fn from(value: bool) -> Self {
        Self::Enable(value)
    }
}

impl From<bool> for with_vectors_selector::SelectorOptions {
    fn from(value: bool) -> Self {
        Self::Enable(value)
    }
}

impl<S: Into<String>> From<S> for DeleteCollection {
    fn from(value: S) -> Self {
        DeleteCollectionBuilder::new(value).build()
    }
}

impl<S: Into<String>> From<S> for CollectionExistsRequest {
    fn from(value: S) -> Self {
        Self {
            collection_name: value.into(),
        }
    }
}

impl<S: Into<String>> From<S> for GetCollectionInfoRequest {
    fn from(value: S) -> Self {
        Self {
            collection_name: value.into(),
        }
    }
}

impl<S: Into<String>> From<S> for DeleteAlias {
    fn from(value: S) -> Self {
        Self {
            alias_name: value.into(),
        }
    }
}

impl<S: Into<String>> From<S> for ListCollectionAliasesRequest {
    fn from(value: S) -> Self {
        Self {
            collection_name: value.into(),
        }
    }
}

impl<S: Into<String>> From<S> for CollectionClusterInfoRequest {
    fn from(value: S) -> Self {
        Self {
            collection_name: value.into(),
        }
    }
}

impl<S: Into<String>> From<S> for CreateSnapshotRequest {
    fn from(value: S) -> Self {
        Self {
            collection_name: value.into(),
        }
    }
}

impl<S: Into<String>> From<S> for ListSnapshotsRequest {
    fn from(value: S) -> Self {
        Self {
            collection_name: value.into(),
        }
    }
}

impl<S: Into<String>> From<S> for DeleteFullSnapshotRequest {
    fn from(value: S) -> Self {
        Self {
            snapshot_name: value.into(),
        }
    }
}
