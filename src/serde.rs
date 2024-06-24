#![allow(deprecated)]

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::client::Payload;
use crate::qdrant::value::Kind;
use crate::qdrant::{ListValue, Struct, Value};

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
