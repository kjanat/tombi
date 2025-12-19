mod array;
mod boolean;
mod date_time;
mod float;
mod integer;
mod string;
mod table;

use crate::IntoDocument;
pub use array::{Array, ArrayKind};
pub use boolean::Boolean;
pub use date_time::{LocalDate, LocalDateTime, LocalTime, OffsetDateTime, TimeZoneOffset};
pub use float::Float;
pub use integer::{Integer, IntegerKind};
use serde::forward_to_deserialize_any;
pub use string::{String, StringKind};
pub use table::{Table, TableKind};

#[cfg(feature = "serde")]
use serde::de::{Deserializer, IntoDeserializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    Boolean,
    Integer,
    Float,
    String,
    OffsetDateTime,
    LocalDateTime,
    LocalDate,
    LocalTime,
    Array,
    Table,
}

impl std::fmt::Display for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean => write!(f, "Boolean"),
            Self::Integer => write!(f, "Integer"),
            Self::Float => write!(f, "Float"),
            Self::String => write!(f, "String"),
            Self::OffsetDateTime => write!(f, "OffsetDateTime"),
            Self::LocalDateTime => write!(f, "LocalDateTime"),
            Self::LocalDate => write!(f, "LocalDate"),
            Self::LocalTime => write!(f, "LocalTime"),
            Self::Array => write!(f, "Array"),
            Self::Table => write!(f, "Table"),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::de::Expected for ValueKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{self}")
    }
}

#[cfg(feature = "serde")]
impl<'a> TryFrom<serde::de::Unexpected<'a>> for ValueKind {
    type Error = serde::de::Unexpected<'a>;

    fn try_from(unexp: serde::de::Unexpected<'a>) -> Result<Self, Self::Error> {
        match unexp {
            serde::de::Unexpected::Bool(_) => Ok(Self::Boolean),
            serde::de::Unexpected::Unsigned(_) => Ok(Self::Integer),
            serde::de::Unexpected::Signed(_) => Ok(Self::Integer),
            serde::de::Unexpected::Float(_) => Ok(Self::Float),
            serde::de::Unexpected::Char(_) => Ok(Self::String),
            serde::de::Unexpected::Str(_) => Ok(Self::String),
            serde::de::Unexpected::Bytes(_) => Ok(Self::String),
            serde::de::Unexpected::Seq => Ok(Self::Array),
            serde::de::Unexpected::Map => Ok(Self::Table),
            serde::de::Unexpected::StructVariant => Ok(Self::String),
            serde::de::Unexpected::Unit
            | serde::de::Unexpected::Option
            | serde::de::Unexpected::NewtypeStruct
            | serde::de::Unexpected::Enum
            | serde::de::Unexpected::UnitVariant
            | serde::de::Unexpected::NewtypeVariant
            | serde::de::Unexpected::TupleVariant
            | serde::de::Unexpected::Other(_) => Err(unexp),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(String),
    OffsetDateTime(OffsetDateTime),
    LocalDateTime(LocalDateTime),
    LocalDate(LocalDate),
    LocalTime(LocalTime),
    Array(Array),
    Table(Table),
}

impl Value {
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> ValueKind {
        match self {
            Self::Boolean(_) => ValueKind::Boolean,
            Self::Integer(_) => ValueKind::Integer,
            Self::Float(_) => ValueKind::Float,
            Self::String(_) => ValueKind::String,
            Self::OffsetDateTime(_) => ValueKind::OffsetDateTime,
            Self::LocalDateTime(_) => ValueKind::LocalDateTime,
            Self::LocalDate(_) => ValueKind::LocalDate,
            Self::LocalTime(_) => ValueKind::LocalTime,
            Self::Array(_) => ValueKind::Array,
            Self::Table(_) => ValueKind::Table,
        }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn unexpected(&self) -> serde::de::Unexpected<'_> {
        match self {
            Self::Boolean(bool) => serde::de::Unexpected::Bool(bool.value()),
            Self::Integer(integer) => serde::de::Unexpected::Signed(integer.value()),
            Self::Float(float) => serde::de::Unexpected::Float(float.value()),
            Self::String(string) => serde::de::Unexpected::Str(string.value()),
            Self::OffsetDateTime(_) => {
                serde::de::Unexpected::Other(tombi_date_time::OffsetDateTime::type_name())
            }
            Self::LocalDateTime(_) => {
                serde::de::Unexpected::Other(tombi_date_time::LocalDateTime::type_name())
            }
            Self::LocalDate(_) => {
                serde::de::Unexpected::Other(tombi_date_time::LocalDate::type_name())
            }
            Self::LocalTime(_) => {
                serde::de::Unexpected::Other(tombi_date_time::LocalTime::type_name())
            }
            Self::Array(_) => serde::de::Unexpected::Seq,
            Self::Table(_) => serde::de::Unexpected::Map,
        }
    }
}

impl IntoDocument<Value> for tombi_document_tree::Value {
    fn into_document(self, toml_version: crate::TomlVersion) -> Value {
        match self {
            Self::Boolean(value) => Value::Boolean(value.into()),
            Self::Integer(value) => Value::Integer(value.into()),
            Self::Float(value) => Value::Float(value.into()),
            Self::String(value) => Value::String(value.into()),
            Self::OffsetDateTime(value) => Value::OffsetDateTime(value.into()),
            Self::LocalDateTime(value) => Value::LocalDateTime(value.into()),
            Self::LocalDate(value) => Value::LocalDate(value.into()),
            Self::LocalTime(value) => Value::LocalTime(value.into()),
            Self::Array(value) => Value::Array(value.into_document(toml_version)),
            Self::Table(value) => Value::Table(value.into_document(toml_version)),
            Self::Incomplete { .. } => {
                unreachable!("Incomplete value should not be converted to document")
            }
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Boolean(value) => value.serialize(serializer),
            Self::Integer(value) => value.serialize(serializer),
            Self::Float(value) => value.serialize(serializer),
            Self::String(value) => value.serialize(serializer),
            Self::OffsetDateTime(value) => value.serialize(serializer),
            Self::LocalDateTime(value) => value.serialize(serializer),
            Self::LocalDate(value) => value.serialize(serializer),
            Self::LocalTime(value) => value.serialize(serializer),
            Self::Array(value) => value.serialize(serializer),
            Self::Table(value) => value.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid Value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Value, E> {
                Ok(Value::Boolean(Boolean::new(v)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Value, E> {
                Ok(Value::Integer(Integer::new(v)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Integer(Integer::new(v as i64)))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Value, E> {
                Ok(Value::Float(Float::new(v)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(String::new(
                    StringKind::BasicString,
                    v.to_string(),
                )))
            }

            fn visit_string<E>(self, v: std::string::String) -> Result<Value, E> {
                Ok(Value::String(String::new(StringKind::BasicString, v)))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut vec = Array::new(ArrayKind::ArrayOfTable);
                while let Some(elem) = seq.next_element()? {
                    vec.push(elem);
                }
                Ok(Value::Array(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut index_map = Table::new(TableKind::Table);
                while let Some((key, value)) = map.next_entry()? {
                    index_map.insert(key, value);
                }
                Ok(Value::Table(index_map))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

macro_rules! deserialize_value {
    ($func_name:ident, $value_type:ident, $visit_method:ident) => {
        fn $func_name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match self {
                Value::$value_type(value) => visitor.$visit_method(value.value()),
                _ => Err(serde::de::Error::invalid_type(
                    self.unexpected(),
                    &ValueKind::$value_type,
                )),
            }
        }
    };
}

impl<'de> serde::de::Deserializer<'de> for &'de Value {
    type Error = crate::de::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.kind() {
            ValueKind::Boolean => self.deserialize_bool(visitor),
            ValueKind::Integer => self.deserialize_i64(visitor),
            ValueKind::Float => self.deserialize_f64(visitor),
            ValueKind::String => self.deserialize_str(visitor),
            ValueKind::OffsetDateTime => {
                self.deserialize_newtype_struct(OffsetDateTime::type_name(), visitor)
            }
            ValueKind::LocalDateTime => {
                self.deserialize_newtype_struct(LocalDateTime::type_name(), visitor)
            }
            ValueKind::LocalDate => {
                self.deserialize_newtype_struct(LocalDate::type_name(), visitor)
            }
            ValueKind::LocalTime => {
                self.deserialize_newtype_struct(LocalTime::type_name(), visitor)
            }
            ValueKind::Array => self.deserialize_seq(visitor),
            ValueKind::Table => self.deserialize_map(visitor),
        }
    }

    deserialize_value!(deserialize_bool, Boolean, visit_bool);
    deserialize_value!(deserialize_i64, Integer, visit_i64);
    deserialize_value!(deserialize_f64, Float, visit_f64);
    deserialize_value!(deserialize_str, String, visit_str);

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            Value::Array(array) => array.deserialize_seq(visitor),
            _ => Err(serde::de::Error::invalid_type(
                self.unexpected(),
                &ValueKind::Array,
            )),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            Value::Table(table) => table.deserialize_map(visitor),
            _ => Err(serde::de::Error::invalid_type(
                self.unexpected(),
                &ValueKind::Table,
            )),
        }
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

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_some(self)
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
        match self {
            Value::Table(table) => table.deserialize_enum(name, variants, visitor),
            Value::String(variant) => visitor.visit_enum(EnumRefDeserializer {
                variant: variant.value(),
                value: None,
            }),
            other => Err(serde::de::Error::invalid_type(
                other.unexpected(),
                &"string or map",
            )),
        }
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i128 u8 u16 u32 u64 u128 f32 char string
        bytes byte_buf unit unit_struct tuple
        tuple_struct struct identifier
    }
}

struct EnumRefDeserializer<'de> {
    variant: &'de str,
    value: Option<&'de Value>,
}

impl<'de> serde::de::EnumAccess<'de> for EnumRefDeserializer<'de> {
    type Error = crate::de::Error;
    type Variant = VariantRefDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantRefDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

struct VariantRefDeserializer<'de> {
    value: Option<&'de Value>,
}

impl<'de> serde::de::VariantAccess<'de> for VariantRefDeserializer<'de> {
    type Error = crate::de::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Some(value) => serde::Deserialize::deserialize(value),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            Some(Value::Array(array)) => {
                if array.is_empty() {
                    visitor.visit_unit()
                } else {
                    array.deserialize_seq(visitor)
                }
            }
            Some(other) => Err(serde::de::Error::invalid_type(
                other.unexpected(),
                &"tuple variant",
            )),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            Some(Value::Table(table)) => table.deserialize_any(visitor),
            Some(other) => Err(serde::de::Error::invalid_type(
                other.unexpected(),
                &"struct variant",
            )),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}
