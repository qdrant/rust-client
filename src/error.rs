#![allow(deprecated)]

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

use crate::qdrant::value::Kind::*;
use crate::qdrant::{ListValue, Struct, Value};

/// An error for failed conversions (e.g. calling [`String::try_from(v)`](String::try_from) on an
/// integer [`Value`])
#[deprecated(since = "1.10.0", note = "new functions don't use this type anymore")]
pub struct NotA<T> {
    marker: PhantomData<T>,
}

impl<T> Default for NotA<T> {
    fn default() -> Self {
        NotA {
            marker: PhantomData,
        }
    }
}
impl Error for NotA<Struct> {}

impl Debug for NotA<Struct> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for NotA<Struct> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("not a Struct")
    }
}

impl Error for NotA<ListValue> {}

impl Debug for NotA<ListValue> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for NotA<ListValue> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("not a ListValue")
    }
}

// Error + Conversion impl for bool
impl Error for NotA<bool> {}

impl Debug for NotA<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for NotA<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(concat!("not a bool"))
    }
}

impl TryFrom<Value> for bool {
    type Error = NotA<bool>;

    fn try_from(v: Value) -> Result<Self, NotA<bool>> {
        if let Some(BoolValue(t)) = v.kind {
            Ok(t)
        } else {
            Err(NotA::default())
        }
    }
}

// Error + Conversion impl for i64
impl Error for NotA<i64> {}

impl Debug for NotA<i64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for NotA<i64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(concat!("not an i64"))
    }
}

impl TryFrom<Value> for i64 {
    type Error = NotA<i64>;

    fn try_from(v: Value) -> Result<Self, NotA<i64>> {
        if let Some(IntegerValue(t)) = v.kind {
            Ok(t)
        } else {
            Err(NotA::default())
        }
    }
}

// Error + Conversion impl for f64
impl Error for NotA<f64> {}

impl Debug for NotA<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for NotA<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(concat!("not a f64"))
    }
}

impl TryFrom<Value> for f64 {
    type Error = NotA<f64>;

    fn try_from(v: Value) -> Result<Self, NotA<f64>> {
        if let Some(DoubleValue(t)) = v.kind {
            Ok(t)
        } else {
            Err(NotA::default())
        }
    }
}

// Error + Conversion impl for string
impl Error for NotA<String> {}

impl Debug for NotA<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for NotA<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(concat!("not a String"))
    }
}

impl TryFrom<Value> for String {
    type Error = NotA<String>;

    fn try_from(v: Value) -> Result<Self, NotA<String>> {
        if let Some(StringValue(t)) = v.kind {
            Ok(t)
        } else {
            Err(NotA::default())
        }
    }
}
