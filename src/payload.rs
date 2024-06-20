use crate::qdrant::Value;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Payload(pub(crate) HashMap<String, Value>);

#[cfg(feature = "serde")]
impl Payload {
    /// Convert a JSON object into a Payload
    ///
    /// Returns `None` if the given JSON value is not an object.
    #[inline]
    pub fn from_json_object(value: serde_json::Value) -> Option<Self> {
        if let serde_json::Value::Object(object) = value {
            Some(object.into())
        } else {
            None
        }
    }
}

impl From<Payload> for HashMap<String, Value> {
    #[inline]
    fn from(payload: Payload) -> Self {
        payload.0
    }
}

impl From<HashMap<&str, Value>> for Payload {
    #[inline]
    fn from(payload: HashMap<&str, Value>) -> Self {
        Self(
            payload
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        )
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Map<String, serde_json::Value>> for Payload {
    #[inline]
    fn from(obj: serde_json::Map<String, serde_json::Value>) -> Self {
        Payload::new_from_hashmap(obj.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl<K, const N: usize> From<[(K, Value); N]> for Payload
where
    K: Into<String>,
{
    fn from(values: [(K, Value); N]) -> Self {
        let mut map = HashMap::with_capacity(N);
        for (k, v) in values.into_iter() {
            map.insert(k.into(), v);
        }
        Self(map)
    }
}

impl Payload {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn new_from_hashmap(payload: HashMap<String, Value>) -> Self {
        Self(payload)
    }

    pub fn insert(&mut self, key: impl ToString, val: impl Into<Value>) {
        self.0.insert(key.to_string(), val.into());
    }
}
