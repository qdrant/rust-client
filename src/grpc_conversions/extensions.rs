use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::client::Payload;
#[allow(deprecated)]
use crate::error::NotA;
use crate::prelude::{PointStruct, Value};
use crate::qdrant::value::Kind;
use crate::qdrant::{
    HardwareUsage, ListValue, PointId, RetrievedPoint, ScoredPoint, Struct, Vectors,
};

/// Null value
static NULL_VALUE: Value = Value {
    kind: Some(Kind::NullValue(0)),
};

impl PointStruct {
    pub fn new(
        id: impl Into<PointId>,
        vectors: impl Into<Vectors>,
        payload: impl Into<Payload>,
    ) -> Self {
        Self {
            id: Some(id.into()),
            payload: payload.into().into(),
            vectors: Some(vectors.into()),
        }
    }
}

impl RetrievedPoint {
    /// Get a payload value for the specified key. If the key is not present,
    /// this will return a null value.
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::RetrievedPoint;
    /// let point = RetrievedPoint::default();
    /// assert!(point.get("not_present").is_null());
    /// ````
    pub fn get(&self, key: &str) -> &Value {
        self.try_get(key).unwrap_or(&NULL_VALUE)
    }

    /// Try to get a payload value for the specified key. If the key is not present,
    /// this will return `None`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::RetrievedPoint;
    /// let point = RetrievedPoint::default();
    /// assert_eq!(point.try_get("not_present"), None);
    /// ````
    pub fn try_get(&self, key: &str) -> Option<&Value> {
        self.payload.get(key)
    }
}

impl ScoredPoint {
    /// Get a payload value for the specified key. If the key is not present,
    /// this will return a null value.
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::ScoredPoint;
    /// let point = ScoredPoint::default();
    /// assert!(point.get("not_present").is_null());
    /// ````
    pub fn get(&self, key: &str) -> &Value {
        self.try_get(key).unwrap_or(&NULL_VALUE)
    }

    /// Get a payload value for the specified key. If the key is not present,
    /// this will return `None`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::ScoredPoint;
    /// let point = ScoredPoint::default();
    /// assert_eq!(point.try_get("not_present"), None);
    /// ````
    pub fn try_get(&self, key: &str) -> Option<&Value> {
        self.payload.get(key)
    }
}

macro_rules! extract {
    ($kind:ident, $check:ident) => {
        /// Check if this value is a
        #[doc = stringify!([$kind])]
        pub fn $check(&self) -> bool {
            matches!(self.kind, Some($kind(_)))
        }
    };
    ($kind:ident, $check:ident, $extract:ident, $ty:ty) => {
        extract!($kind, $check);

        /// Get this value as
        #[doc = stringify!([$ty])]
        ///
        /// Returns `None` if this value is not a
        #[doc = stringify!([$kind].)]
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

        /// Get this value as
        #[doc = stringify!([$ty])]
        ///
        /// Returns `None` if this value is not a
        #[doc = stringify!([$kind].)]
        pub fn $extract(&self) -> Option<&$ty> {
            if let Some($kind(v)) = &self.kind {
                Some(v)
            } else {
                None
            }
        }
    };
}

// Separate module to not import all enum kinds of `Kind` directly as this conflicts with other types.
// The macro extract!() however is built to take enum kinds directly and passing Kind::<kind> is not possible.
mod value_extract_impl {
    use crate::qdrant::value::Kind::*;
    use crate::qdrant::{Struct, Value};
    impl Value {
        extract!(NullValue, is_null);
        extract!(BoolValue, is_bool, as_bool, bool);
        extract!(IntegerValue, is_integer, as_integer, i64);
        extract!(DoubleValue, is_double, as_double, f64);
        extract!(StringValue, is_str, as_str, ref String);
        extract!(ListValue, is_list, as_list, ref [Value]);
        extract!(StructValue, is_struct, as_struct, ref Struct);
    }
}

impl Value {
    #[cfg(feature = "serde")]
    /// Convert this into a [`serde_json::Value`]
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
            Some(Kind::BoolValue(b)) => JsonValue::Bool(b),
            Some(Kind::IntegerValue(i)) => JsonValue::from(i),
            Some(Kind::DoubleValue(d)) => JsonValue::from(d),
            Some(Kind::StringValue(s)) => JsonValue::String(s),
            Some(Kind::ListValue(vs)) => vs.into_iter().map(Value::into_json).collect(),
            Some(Kind::StructValue(s)) => s
                .fields
                .into_iter()
                .map(|(k, v)| (k, v.into_json()))
                .collect(),
            Some(Kind::NullValue(_)) | None => JsonValue::Null,
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
            Some(Kind::BoolValue(b)) => write!(f, "{}", b),
            Some(Kind::IntegerValue(i)) => write!(f, "{}", i),
            Some(Kind::DoubleValue(v)) => write!(f, "{}", v),
            Some(Kind::StringValue(s)) => write!(f, "{:?}", s),
            Some(Kind::ListValue(vs)) => {
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
            Some(Kind::StructValue(s)) => {
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
    /// Try to get an iterator over the items of the contained list value
    ///
    /// Returns `None` if this is not a list.
    pub fn try_list_iter(&self) -> Option<impl Iterator<Item = &Value>> {
        if let Some(Kind::ListValue(values)) = &self.kind {
            Some(values.iter())
        } else {
            None
        }
    }

    /// Try to get an iterator over the items of the contained list value, if any
    #[deprecated(since = "1.10.0", note = "use `try_list_iter` instead")]
    #[allow(deprecated)]
    pub fn iter_list(&self) -> Result<impl Iterator<Item = &Value>, NotA<ListValue>> {
        if let Some(Kind::ListValue(values)) = &self.kind {
            Ok(values.iter())
        } else {
            Err(NotA::default())
        }
    }

    /// Get a value from a struct field
    ///
    /// Returns `None` if this is not a struct type or if the field is not present.
    pub fn get_value(&self, key: &str) -> Option<&Value> {
        if let Some(Kind::StructValue(Struct { fields })) = &self.kind {
            Some(fields.get(key)?)
        } else {
            None
        }
    }

    /// Try to get a field from the struct if this value contains one
    #[deprecated(since = "1.10.0", note = "use `get_value` instead")]
    #[allow(deprecated)]
    pub fn get_struct(&self, key: &str) -> Result<&Value, NotA<Struct>> {
        if let Some(Kind::StructValue(Struct { fields })) = &self.kind {
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

impl Hash for PointId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use crate::qdrant::point_id::PointIdOptions::{Num, Uuid};
        match &self.point_id_options {
            Some(Num(u)) => state.write_u64(*u),
            Some(Uuid(s)) => s.hash(state),
            None => {}
        }
    }
}

impl Hash for ScoredPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Hash for RetrievedPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl HardwareUsage {
    pub(crate) fn aggregate_opts(this: &Option<Self>, other: &Option<Self>) -> Option<Self> {
        match (this, other) {
            (Some(this), Some(other)) => Some(this.aggregate(other)),
            (Some(this), None) => Some(*this),
            (None, Some(other)) => Some(*other),
            (None, None) => None,
        }
    }

    pub(crate) fn aggregate(&self, other: &Self) -> Self {
        let Self {
            cpu,
            payload_io_read,
            payload_io_write,
            payload_index_io_read,
            payload_index_io_write,
            vector_io_read,
            vector_io_write,
        } = other;

        Self {
            cpu: self.cpu + cpu,
            payload_io_read: self.payload_io_read + payload_io_read,
            payload_io_write: self.payload_io_write + payload_io_write,
            payload_index_io_read: self.payload_index_io_read + payload_index_io_read,
            payload_index_io_write: self.payload_index_io_write + payload_index_io_write,
            vector_io_read: self.vector_io_read + vector_io_read,
            vector_io_write: self.vector_io_write + vector_io_write,
        }
    }
}
