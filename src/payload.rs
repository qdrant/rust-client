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

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Payload;
    use serde_json::json;

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
