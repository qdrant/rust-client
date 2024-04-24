use crate::client::Payload;
use crate::qdrant::point_id::PointIdOptions;
use crate::qdrant::points_selector::PointsSelectorOneOf;
use crate::qdrant::value::Kind;
use crate::qdrant::vectors::VectorsOptions;
use crate::qdrant::with_payload_selector::SelectorOptions;
use crate::qdrant::{
    shard_key, with_vectors_selector, ListValue, NamedVectors, PayloadIncludeSelector, PointId,
    PointStruct, PointsIdsList, PointsSelector, ShardKey, ShardKeySelector, SparseIndices, Struct,
    Value, Vector, Vectors, VectorsSelector, WithPayloadSelector, WithVectorsSelector,
};
use std::collections::HashMap;

impl From<bool> for WithPayloadSelector {
    fn from(flag: bool) -> Self {
        WithPayloadSelector {
            selector_options: Some(SelectorOptions::Enable(flag)),
        }
    }
}

impl From<Vec<&str>> for WithPayloadSelector {
    fn from(fields: Vec<&str>) -> Self {
        WithPayloadSelector {
            selector_options: Some(SelectorOptions::Include(PayloadIncludeSelector {
                fields: fields.into_iter().map(|f| f.to_string()).collect(),
            })),
        }
    }
}

impl From<Vec<PointId>> for PointsSelector {
    fn from(point_ids: Vec<PointId>) -> Self {
        PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                ids: point_ids,
            })),
        }
    }
}

impl From<Vec<f32>> for Vector {
    fn from(vector: Vec<f32>) -> Self {
        Vector {
            data: vector,
            indices: None,
        }
    }
}

impl From<Vec<(u32, f32)>> for Vector {
    fn from(tuples: Vec<(u32, f32)>) -> Self {
        Self::from(tuples.as_slice())
    }
}

// Since we construct two new Vec's anyway it's fine to source from a reference
impl From<&[(u32, f32)]> for Vector {
    fn from(tuples: &[(u32, f32)]) -> Self {
        let mut indices = Vec::with_capacity(tuples.len());
        let mut values = Vec::with_capacity(tuples.len());
        for (i, w) in tuples {
            indices.push(*i);
            values.push(*w);
        }
        Vector {
            data: values,
            indices: Some(SparseIndices { data: indices }),
        }
    }
}

impl From<HashMap<String, Vec<f32>>> for Vectors {
    fn from(named_vectors: HashMap<String, Vec<f32>>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

impl From<HashMap<String, Vector>> for Vectors {
    fn from(named_vectors: HashMap<String, Vector>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors.into_iter().collect(),
            })),
        }
    }
}

impl From<HashMap<String, Vec<(u32, f32)>>> for Vectors {
    fn from(named_vectors: HashMap<String, Vec<(u32, f32)>>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

// Since we construct two new Vec's anyway it's fine to source from a reference
impl From<HashMap<String, &[(u32, f32)]>> for Vectors {
    fn from(named_vectors: HashMap<String, &[(u32, f32)]>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

impl From<Vec<f32>> for Vectors {
    fn from(vector: Vec<f32>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vector(vector.into())),
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

impl From<Payload> for Value {
    fn from(val: Payload) -> Self {
        Self {
            kind: Some(Kind::StructValue(Struct { fields: val.0 })),
        }
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(val: Vec<T>) -> Self {
        Self {
            kind: Some(Kind::ListValue(ListValue {
                values: val.into_iter().map(|v| v.into()).collect(),
            })),
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

impl From<shard_key::Key> for ShardKey {
    fn from(key: shard_key::Key) -> Self {
        ShardKey { key: Some(key) }
    }
}

impl From<Vec<shard_key::Key>> for ShardKeySelector {
    fn from(shard_keys: Vec<shard_key::Key>) -> Self {
        ShardKeySelector {
            shard_keys: shard_keys.into_iter().map(ShardKey::from).collect(),
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

impl From<String> for ShardKeySelector {
    fn from(keyword: String) -> Self {
        keyword.into()
    }
}

impl From<u64> for ShardKeySelector {
    fn from(number: u64) -> Self {
        number.into()
    }
}

impl From<Vec<String>> for ShardKeySelector {
    fn from(keywords: Vec<String>) -> Self {
        keywords.into()
    }
}

impl From<Vec<u64>> for ShardKeySelector {
    fn from(numbers: Vec<u64>) -> Self {
        numbers.into()
    }
}

impl From<String> for PointId {
    fn from(val: String) -> Self {
        Self {
            point_id_options: Some(PointIdOptions::Uuid(val)),
        }
    }
}

impl From<u64> for PointId {
    fn from(val: u64) -> Self {
        Self {
            point_id_options: Some(PointIdOptions::Num(val)),
        }
    }
}

impl PointStruct {
    pub fn new(id: impl Into<PointId>, vectors: impl Into<Vectors>, payload: Payload) -> Self {
        Self {
            id: Some(id.into()),
            payload: payload.into(),
            vectors: Some(vectors.into()),
        }
    }
}
