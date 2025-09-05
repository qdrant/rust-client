//! Deserialize into any serde type
//!
//! Sourced from <https://github.com/serde-rs/json/blob/master/src/value/de.rs>

use std::collections::hash_map::IntoIter;

use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::{
    DeserializeSeed, EnumAccess, Expected, IntoDeserializer, Unexpected, VariantAccess, Visitor,
};
use serde::Deserializer;

use crate::qdrant::value::Kind;
use crate::qdrant::{Struct, Value};

#[derive(PartialEq, Eq, Clone)]
pub enum DeserPayloadError {
    Error(String),
    IntegerConversion { got: i64, max: i64 },
}

impl std::fmt::Debug for DeserPayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserPayloadError::Error(error) => write!(f, "{error}"),
            DeserPayloadError::IntegerConversion { got, max } => {
                write!(f, "Overflowing integer conversion: {got} > {max}")
            }
        }
    }
}

impl std::fmt::Display for DeserPayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for DeserPayloadError {}

impl serde::de::Error for DeserPayloadError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Error(msg.to_string())
    }
}

impl<'de> IntoDeserializer<'de, DeserPayloadError> for Struct {
    type Deserializer = MapDeserializer<'de, IntoIter<String, Value>, DeserPayloadError>;

    #[inline]
    fn into_deserializer(self) -> Self::Deserializer {
        MapDeserializer::new(self.fields.into_iter())
    }
}

impl<'de> IntoDeserializer<'de, DeserPayloadError> for Value {
    type Deserializer = Self;

    #[inline]
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! deserialize_number {
    ($method:ident,$visitorfn:ident,$numsize:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, DeserPayloadError>
        where
            V: Visitor<'de>,
        {
            match self.kind.as_ref() {
                Some(Kind::IntegerValue(n)) => {
                    visitor.$visitorfn($numsize::try_from(*n).map_err(|_| {
                        DeserPayloadError::IntegerConversion {
                            got: *n,
                            max: $numsize::MAX as i64,
                        }
                    })?)
                }
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    };
}

fn visit_array<'de, V>(array: Vec<Value>, visitor: V) -> Result<V::Value, DeserPayloadError>
where
    V: Visitor<'de>,
{
    let mut deserializer = SeqDeserializer::new(array.into_iter());
    let seq = visitor.visit_seq(&mut deserializer)?;
    deserializer.end()?;
    Ok(seq)
}

impl<'de> Deserializer<'de> for Value {
    type Error = DeserPayloadError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind {
            Some(kind) => match kind {
                Kind::NullValue(_) => visitor.visit_unit(),
                Kind::DoubleValue(double) => visitor.visit_f64(double),
                Kind::IntegerValue(n) => visitor.visit_i64(n),
                Kind::StringValue(s) => visitor.visit_string(s),
                Kind::BoolValue(b) => visitor.visit_bool(b),
                Kind::StructValue(s) => s.into_deserializer().deserialize_any(visitor),
                Kind::ListValue(list_value) => visit_array(list_value.values, visitor),
            },
            None => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind {
            Some(Kind::BoolValue(b)) => visitor.visit_bool(b),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    deserialize_number!(deserialize_i8, visit_i8, i8);
    deserialize_number!(deserialize_i16, visit_i16, i16);
    deserialize_number!(deserialize_i32, visit_i32, i32);
    deserialize_number!(deserialize_i64, visit_i64, i64);
    deserialize_number!(deserialize_u8, visit_u8, u8);
    deserialize_number!(deserialize_u16, visit_u16, u16);
    deserialize_number!(deserialize_u32, visit_u32, u32);
    deserialize_number!(deserialize_u64, visit_u64, u64);

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, DeserPayloadError>
    where
        V: Visitor<'de>,
    {
        match self.kind.as_ref() {
            Some(Kind::DoubleValue(n)) => visitor.visit_f32(*n as f32),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, DeserPayloadError>
    where
        V: Visitor<'de>,
    {
        match self.kind.as_ref() {
            Some(Kind::DoubleValue(n)) => visitor.visit_f64(*n),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind {
            Some(Kind::StringValue(b)) => visitor.visit_str(&b),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind {
            Some(Kind::StringValue(b)) => visitor.visit_string(b),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind {
            Some(Kind::StringValue(b)) => visitor.visit_string(b),
            Some(Kind::ListValue(b)) => visit_array(b.values, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.kind.is_none() || matches!(self.kind, Some(Kind::NullValue(..))) {
            return visitor.visit_none();
        }

        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match &self.kind {
            Some(Kind::NullValue(_)) => visitor.visit_unit(),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind {
            Some(Kind::ListValue(list)) => visit_array(list.values, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind {
            Some(Kind::StructValue(r#struct)) => {
                r#struct.into_deserializer().deserialize_any(visitor)
            }
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind {
            Some(k) => match k {
                Kind::StructValue(r#struct) => {
                    r#struct.into_deserializer().deserialize_any(visitor)
                }
                Kind::ListValue(list) => visit_array(list.values, visitor),
                _ => Err(Self::invalid_type_from_kind(&k, &visitor)),
            },
            None => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind {
            Some(k) => match k {
                Kind::StructValue(r#struct) => r#struct
                    .into_deserializer()
                    .deserialize_enum(name, variants, visitor),
                Kind::StringValue(variant) => visitor.visit_enum(EnumDeserializer { variant }),
                _ => Err(Self::invalid_type_from_kind(&k, &visitor)),
            },
            None => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        drop(self);
        visitor.visit_unit()
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(serde::de::Error::custom("i128 is not supported"))
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(serde::de::Error::custom("u128 is not supported"))
    }
}

struct EnumDeserializer {
    variant: String,
}

impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = DeserPayloadError;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantDeserializer), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantDeserializer;
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

struct VariantDeserializer;

impl<'de> VariantAccess<'de> for VariantDeserializer {
    type Error = DeserPayloadError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}

impl Value {
    #[cold]
    fn invalid_type<E>(&self, exp: &dyn Expected) -> E
    where
        E: serde::de::Error,
    {
        serde::de::Error::invalid_type(self.unexpected(), exp)
    }

    #[cold]
    fn unexpected<'a>(&'a self) -> Unexpected<'a> {
        match self.kind.as_ref() {
            Some(k) => match k {
                Kind::NullValue(_) => Unexpected::Unit,
                Kind::DoubleValue(d) => Unexpected::Float(*d),
                Kind::IntegerValue(i) => Unexpected::Signed(*i),
                Kind::StringValue(s) => Unexpected::Str(s),
                Kind::BoolValue(b) => Unexpected::Bool(*b),
                Kind::StructValue(_) => Unexpected::Map,
                Kind::ListValue(_) => Unexpected::Seq,
            },
            None => Unexpected::Unit,
        }
    }

    #[cold]
    fn invalid_type_from_kind<E>(kind: &Kind, exp: &dyn Expected) -> E
    where
        E: serde::de::Error,
    {
        serde::de::Error::invalid_type(Self::unexpected_kind(kind), exp)
    }

    #[cold]
    fn unexpected_kind<'a>(kind: &'a Kind) -> Unexpected<'a> {
        match kind {
            Kind::NullValue(_) => Unexpected::Unit,
            Kind::DoubleValue(d) => Unexpected::Float(*d),
            Kind::IntegerValue(i) => Unexpected::Signed(*i),
            Kind::StringValue(s) => Unexpected::Str(s),
            Kind::BoolValue(b) => Unexpected::Bool(*b),
            Kind::StructValue(_) => Unexpected::Map,
            Kind::ListValue(_) => Unexpected::Seq,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, HashMap};

    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::{serde_deser::DeserPayloadError, Payload};

    #[test]
    fn test_json_deser() {
        #[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
        struct S {
            some_string: String,
            some_bool: bool,
            some_int: i32,
            some_seq: Vec<String>,
            some_obj: HashMap<String, String>,
            tuple: (String, u32),
            #[allow(clippy::type_complexity)]
            tuple_map: BTreeMap<String, (String, u32, Vec<(u32, String)>)>,
        }

        let value = S {
            some_string: "Bar".into(),
            some_bool: true,
            some_int: 12,
            some_seq: vec!["elem1".into(), "elem2".into()],
            some_obj: HashMap::from([("key".into(), "value".into())]),
            tuple: ("abc".to_string(), 42),
            tuple_map: BTreeMap::from([
                (
                    "key1".to_string(),
                    ("42".to_string(), 13, vec![(43, "abc".to_string())]),
                ),
                (
                    "key2".to_string(),
                    ("rust".to_string(), 91, vec![(84, "python".to_string())]),
                ),
            ]),
        };

        let json_value = serde_json::to_value(&value).unwrap();
        let payload: Payload = json_value.try_into().unwrap();
        let deserialized_value: S = payload.deserialize().unwrap();

        assert_eq!(value, deserialized_value)
    }

    fn make_payload(val: serde_json::Value) -> Payload {
        Payload::from(val.as_object().expect("Can only make a json object to Payload. This is a bug in the unit test itself.").clone())
    }

    #[test]
    fn test_integer_conversion() {
        let payload = make_payload(json!({"integer": u32::MAX}));

        #[derive(Deserialize, Clone)]
        #[allow(dead_code)]
        struct DstU8 {
            integer: u8,
        }

        // Check fail
        let got: Result<DstU8, _> = payload.clone().deserialize();
        assert_eq!(
            got.err().unwrap().as_payload_deserialization().unwrap(),
            &DeserPayloadError::IntegerConversion {
                got: u32::MAX as i64,
                max: u8::MAX as i64
            }
        );

        #[derive(Deserialize, Debug, PartialEq)]
        #[allow(dead_code)]
        struct DstU32 {
            integer: u32,
        }

        // Check success
        let got: Result<DstU32, _> = payload.deserialize();
        assert_eq!(got.unwrap(), DstU32 { integer: u32::MAX });
    }

    #[test]
    fn test_nested() {
        let payload = make_payload(json!({"number": 42, "data": {"text": "abc", "num": 99}}));

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct Dst {
            number: u32,
            data: DstData,
        }

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct DstData {
            text: String,
            num: u8,
        }

        let dst: Dst = payload.deserialize().unwrap();
        assert_eq!(dst.number, 42);
        assert_eq!(dst.data.num, 99);
        assert_eq!(dst.data.text, "abc");
    }

    #[test]
    fn test_tuple() {
        let payload = make_payload(json!({"number": 42, "seq": [1,2,3]}));

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct Dst {
            number: u32,
            seq: (u8, u8, u8),
        }

        let dst: Dst = payload.clone().deserialize().unwrap();
        assert_eq!(dst.seq, (1, 2, 3));

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct DstTooShort {
            number: u32,
            seq: (u8, u8),
        }

        let dst: Result<DstTooShort, _> = payload.clone().deserialize();
        assert!(dst.is_err());

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct DstTooLong {
            number: u32,
            seq: (u8, u8, u8, u8),
        }

        let dst: Result<DstTooLong, _> = payload.deserialize();
        assert!(dst.is_err());
    }

    #[test]
    fn test_enum() {
        let payload = make_payload(json!({"items": ["Major","Minor","Mid"]}));

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)]
        struct Dst {
            items: Vec<DstEnum>,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        #[allow(dead_code)]
        enum DstEnum {
            Major,
            Minor,
            Mid,
            SomewhereElse,
        }

        let dst: Dst = payload.deserialize().unwrap();
        assert_eq!(
            dst.items,
            vec![DstEnum::Major, DstEnum::Minor, DstEnum::Mid]
        );

        let payload = make_payload(json!({"items": ["Major","Minor","Middle"]})); // "Middle" is no enum variant.
        let dst: Result<Dst, _> = payload.deserialize();
        assert!(dst.is_err())
    }

    #[test]
    fn test_enum_tuple_struct() {
        let payload = make_payload(json!({"items": ["Major",32,true]}));

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)]
        struct Dst {
            items: Vec<DstEnum>,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        #[allow(dead_code)]
        #[serde(untagged)]
        enum DstEnum {
            Str(String),
            Num(i32),
            Bool(bool),
        }

        let dst: Dst = payload.deserialize().unwrap();
        assert_eq!(
            dst.items,
            vec![
                DstEnum::Str("Major".to_string()),
                DstEnum::Num(32),
                DstEnum::Bool(true)
            ]
        );
    }

    #[test]
    fn test_enum_struct() {
        let payload = make_payload(json!({"items": [{"key": "Major"},{"other": 32},true]}));

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)]
        struct Dst {
            items: Vec<DstEnum>,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        #[allow(dead_code)]
        #[serde(untagged)]
        enum DstEnum {
            First { key: String },
            Second { other: u32 },
            Bool(bool),
        }

        let dst: Dst = payload.deserialize().unwrap();
        assert_eq!(
            dst.items,
            vec![
                DstEnum::First {
                    key: "Major".to_string()
                },
                DstEnum::Second { other: 32 },
                DstEnum::Bool(true)
            ]
        );

        let payload_direct =
            make_payload(json!({"item": {"key": "Major"}, "other": {"other": 32}, "third": false}));

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)]
        struct Dst2 {
            item: DstEnum,
            other: DstEnum,
            third: DstEnum,
        }

        let dst: Dst2 = payload_direct.deserialize().unwrap();
        assert_eq!(
            dst.item,
            DstEnum::First {
                key: "Major".to_string()
            }
        );
        assert_eq!(dst.other, DstEnum::Second { other: 32 });
        assert_eq!(dst.third, DstEnum::Bool(false));

        let dst_err: Result<Dst2, _> = make_payload(
            json!({"item": {"key2": "Major"}, "other": {"other": 32}, "third": false}),
        )
        .deserialize();
        assert!(dst_err.is_err())
    }

    #[test]
    fn test_enum_struct_tagged() {
        let payload = make_payload(
            json!({"items": [{"t": "First", "c": {"key": "Major"}}, {"t": "Second","c": {"other": 32}},{"t": "Bool", "c": true}]}),
        );

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)]
        struct Dst {
            items: Vec<DstEnum>,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        #[allow(dead_code)]
        #[serde(tag = "t", content = "c")]
        enum DstEnum {
            First { key: String },
            Second { other: u32 },
            Bool(bool),
        }

        let dst: Dst = payload.deserialize().unwrap();
        assert_eq!(
            dst.items,
            vec![
                DstEnum::First {
                    key: "Major".to_string()
                },
                DstEnum::Second { other: 32 },
                DstEnum::Bool(true)
            ]
        );
    }

    #[test]
    fn test_option() {
        let payload = make_payload(json!({
            "str1": "hi",
            "int1": 32,
            "vec1": [42]
        }));

        #[derive(Deserialize, Debug, PartialEq)]
        #[allow(dead_code)]
        struct Dst {
            str1: Option<String>,
            str2: Option<String>,
            int1: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            int2: Option<i32>,
            vec1: Option<Vec<u32>>,
            vec2: Option<Vec<u32>>,
        }

        let dst: Dst = payload.deserialize().unwrap();
        assert_eq!(
            dst,
            Dst {
                str1: Some("hi".to_string()),
                str2: None,
                int1: Some(32),
                int2: None,
                vec1: Some(vec![42]),
                vec2: None,
            }
        )
    }

    #[test]
    fn test_other_types() {
        let payload = make_payload(json!({
            "bytes": [0,4,15,11,96,12],
            "double": 0.2,
            "null": None::<u32>,
            "single_char": 'c',
        }));

        #[derive(Deserialize, Debug, PartialEq)]
        #[allow(dead_code)]
        struct Dst {
            bytes: Vec<u8>,
            double: f32,
            null: (),
            single_char: char,
        }

        let dst: Dst = payload.deserialize().unwrap();
        assert_eq!(dst.bytes, vec![0, 4, 15, 11, 96, 12]);
        assert_eq!(dst.double, 0.2);
        assert_eq!(dst.single_char, 'c');
    }
}
