use crate::qdrant::Value;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Payload(pub(crate) HashMap<String, Value>);

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
