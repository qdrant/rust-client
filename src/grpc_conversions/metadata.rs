use std::collections::HashMap;

use crate::qdrant::Value;

/// Proxy structure for simpler conversion from `HashMap<String, serde_json::Value>` to `HashMap<String, Value>`.
pub struct MetadataWrapper(pub HashMap<String, Value>);

#[cfg(feature = "serde")]
impl From<HashMap<String, serde_json::Value>> for MetadataWrapper {
    fn from(value: HashMap<String, serde_json::Value>) -> Self {
        Self(value.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}
