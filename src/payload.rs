use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::qdrant::Value;

/// Point payload
///
/// A JSON-like object that can be attached to points. With payloads you can store any kind of
/// information along with your points. Qdrant provides comprehensive ways to filter on payload
/// values during vector search.
///
/// Payload documentation: <https://qdrant.tech/documentation/concepts/payload/>
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Payload(pub(crate) HashMap<String, Value>);

impl Payload {
    /// Construct a new empty payload object
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Construct a payload object from the given hash map
    #[deprecated(since = "1.10.0", note = "use `Payload::from` instead")]
    pub fn new_from_hashmap(payload: HashMap<String, Value>) -> Self {
        Self(payload)
    }

    /// Convert a JSON object into payload
    ///
    /// Returns `None` if the given JSON value is not an object.
    #[cfg(feature = "serde")]
    #[inline]
    pub fn from_json_object(value: serde_json::Value) -> Option<Self> {
        if let serde_json::Value::Object(object) = value {
            Some(object.into())
        } else {
            None
        }
    }

    /// Insert a payload value at the given key, replacing any existing value
    pub fn insert(&mut self, key: impl ToString, val: impl Into<Value>) {
        self.0.insert(key.to_string(), val.into());
    }
}

impl From<HashMap<String, Value>> for Payload {
    #[inline]
    fn from(payload: HashMap<String, Value>) -> Self {
        Self(payload)
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

impl From<Payload> for HashMap<String, Value> {
    #[inline]
    fn from(payload: Payload) -> Self {
        payload.0
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Map<String, serde_json::Value>> for Payload {
    #[inline]
    fn from(obj: serde_json::Map<String, serde_json::Value>) -> Self {
        Payload::from(
            obj.into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect::<HashMap<String, Value>>(),
        )
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

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::client::Payload;

    #[test]
    fn json_payload_round_trip() {
        let payload: Payload = vec![
            ("some_string", "Bar".into()),
            ("some_bool", true.into()),
            ("some_int", 12.into()),
            ("some_float", 2.3.into()),
            ("some_seq", vec!["elem1", "elem2"].into()),
            ("some_obj", vec![("key", "value")].into()),
        ]
        .into_iter()
        .collect::<HashMap<_, Value>>()
        .into();

        // payload -> Json string
        let json_value = serde_json::to_string(&payload).unwrap();

        // Json string -> payload
        let payload_back: Payload = serde_json::from_str(&json_value).unwrap();

        // assert round trip
        assert_eq!(payload, payload_back);
    }

    #[test]
    fn payload_from_string() {
        let json = r#"{
            "some_string": "Bar",
            "some_bool": true,
            "some_int": 12,
            "some_float": 2.3,
            "some_seq": ["elem1", "elem2"],
            "some_obj": {"key": "value"}
            }"#;

        // String -> payload
        let parsed_payload: Payload = serde_json::from_str(json).unwrap();

        let expected: Payload = vec![
            ("some_string", "Bar".into()),
            ("some_bool", true.into()),
            ("some_int", 12.into()),
            ("some_float", 2.3.into()),
            ("some_seq", vec!["elem1", "elem2"].into()),
            ("some_obj", vec![("key", "value")].into()),
        ]
        .into_iter()
        .collect::<HashMap<_, Value>>()
        .into();

        // assert expected
        assert_eq!(parsed_payload, expected);
    }

    #[test]
    fn test_json_macro() {
        let json_value = json!({
            "some_string": "Bar",
            "some_bool": true,
            "some_int": 12,
            "some_float": 2.3,
            "some_seq": ["elem1", "elem2"],
            "some_obj": {"key": "value"}
        });

        let payload: Payload = Payload::from_json_object(json_value).unwrap();

        eprintln!("payload = {:#?}", payload);
    }
}
