use crate::client::Payload;
use crate::qdrant::value::Kind;
use crate::qdrant::{ListValue, Struct, Value};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct PayloadConversionError(serde_json::Value);

impl Display for PayloadConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to convert json {} to payload: expected object at the top level",
            self.0
        )
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = &self.kind {
            match value {
                Kind::NullValue(_v) => serializer.serialize_none(),
                Kind::DoubleValue(double) => serializer.serialize_f64(*double),
                Kind::IntegerValue(int) => serializer.serialize_i64(*int),
                Kind::StringValue(str) => serializer.serialize_str(str),
                Kind::BoolValue(bool) => serializer.serialize_bool(*bool),
                Kind::StructValue(struc) => {
                    let mut map = serializer.serialize_map(Some(struc.fields.len()))?;
                    for (k, v) in struc.fields.iter() {
                        map.serialize_entry(k, v)?;
                    }
                    map.end()
                }
                Kind::ListValue(list) => {
                    let mut seq = serializer.serialize_seq(Some(list.values.len()))?;
                    for v in &list.values {
                        seq.serialize_element(v)?;
                    }
                    seq.end()
                }
            }
        } else {
            serializer.serialize_none()
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Self {
                kind: Some(Kind::NullValue(0)),
            },
            serde_json::Value::Bool(bool) => Self {
                kind: Some(Kind::BoolValue(bool)),
            },
            serde_json::Value::Number(num) => {
                if let Some(num) = num.as_i64() {
                    Self {
                        kind: Some(Kind::IntegerValue(num)),
                    }
                } else if let Some(num) = num.as_f64() {
                    Self {
                        kind: Some(Kind::DoubleValue(num)),
                    }
                } else {
                    Self {
                        kind: Some(Kind::DoubleValue(f64::NAN)),
                    }
                }
            }
            serde_json::Value::String(str) => Self {
                kind: Some(Kind::StringValue(str)),
            },
            serde_json::Value::Array(arr) => Self {
                kind: Some(Kind::ListValue(ListValue {
                    values: arr.into_iter().map(|v| v.into()).collect(),
                })),
            },
            serde_json::Value::Object(obj) => Self {
                kind: Some(Kind::StructValue(Struct {
                    fields: obj
                        .into_iter()
                        .map(|(k, v)| (k, v.into()))
                        .collect::<HashMap<_, _>>(),
                })),
            },
        }
    }
}

impl TryFrom<serde_json::Value> for Payload {
    type Error = PayloadConversionError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if let serde_json::Value::Object(object) = value {
            Ok(object.into())
        } else {
            Err(PayloadConversionError(value))
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // rely on serde_json to materialize a JSON value for conversion
        let serde_value = serde_json::Value::deserialize(deserializer)?;
        Ok(serde_value.into())
    }
}

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

        let payload: Payload = json_value.try_into().unwrap();

        eprintln!("payload = {:#?}", payload);
    }
}
