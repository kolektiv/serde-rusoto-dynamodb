use super::result::{Error, Result};
use rusoto_dynamodb::AttributeValue;
use serde::de::{Deserialize, Deserializer, Visitor};

// Attribute Value Deserializer

#[derive(new)]
pub struct AttributeValueDeserializer<'de> {
    value: &'de AttributeValue,
}

impl<'de, 'a> Deserializer<'de> for &'a mut AttributeValueDeserializer<'de> {
    type Error = Error;

    // Any

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue { bool: Some(b), .. } => visitor.visit_bool(*b),
            AttributeValue { l: Some(l), .. } => {
                visitor.visit_seq(AttributeValueSeqDeserializer::new(l))
            }
            AttributeValue { m: Some(m), .. } => {
                visitor.visit_map(AttributeValueMapDeserializer::new(m))
            }
            AttributeValue { n: Some(n), .. } => match n.parse::<i64>() {
                Ok(n) => visitor.visit_i64(n),
                _ => match n.parse::<f64>() {
                    Ok(n) => visitor.visit_f64(n),
                    _ => Err(Error::new("Numeric Value Expected")),
                },
            },
            AttributeValue { null: Some(_), .. } => visitor.visit_unit(),
            AttributeValue { s: Some(s), .. } => visitor.visit_borrowed_str(s),
            _ => Err(Error::new("Supported Value Expected")),
        }
    }

    forward_to_deserialize_any! {
        bool f32 f64 i8 i16 i32 i64 identifier ignored_any map seq str string
        struct tuple tuple_struct u8 u16 u32 u64 unit unit_struct
    }

    // Character

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue { s: Some(cstr), .. } => visitor.visit_char(
                cstr.chars()
                    .next()
                    .ok_or_else(|| Error::new("Non-Zero Length String Expected"))?,
            ),
            _ => Err(Error::new("String Value Expected (Char)")),
        }
    }

    // Bytes

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue { b: Some(b), .. } => visitor.visit_bytes(&b[..]),
            _ => Err(Error::new("Byte Vector Value Expected")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue { b: Some(b), .. } => visitor.visit_byte_buf(b.to_vec()),
            _ => Err(Error::new("Byte Vector Value Expected")),
        }
    }

    // Option

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue {
                null: Some(true), ..
            } => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    // Newtype

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Enum

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue { m: Some(m), .. } => match (m.keys().next(), m.values().next()) {
                (Some(key), Some(value)) => {
                    visitor.visit_enum(AttributeValueEnumDeserializer::new(key, value))
                }
                _ => Err(Error::new("Key/Value Expected")),
            },
            _ => Err(Error::new("Map Value Expected")),
        }
    }
}

// =============================================================================

// Compound Deserializers

use serde::{de::DeserializeSeed, forward_to_deserialize_any};

// -----------------------------------------------------------------------------

// Attribute Value Enum Deserializer

use serde::de::EnumAccess;

#[derive(new)]
pub struct AttributeValueEnumDeserializer<'de> {
    key: &'de str,
    value: &'de AttributeValue,
}

impl<'de> EnumAccess<'de> for AttributeValueEnumDeserializer<'de> {
    type Error = Error;
    type Variant = AttributeValueVariantDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        Ok((
            seed.deserialize(AttributeValueEnumKeyDeserializer::new(self.key))?,
            AttributeValueVariantDeserializer::new(self.value),
        ))
    }
}

#[derive(new)]
struct AttributeValueEnumKeyDeserializer<'de> {
    key: &'de str,
}

impl<'de> Deserializer<'de> for AttributeValueEnumKeyDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.key.to_owned())
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct tuple_struct struct
        tuple enum identifier ignored_any
    }
}

// -----------------------------------------------------------------------------

// Attribute Value Map Deserializer

use serde::de::MapAccess;
use std::collections::{
    hash_map::{Keys, Values},
    HashMap,
};

pub struct AttributeValueMapDeserializer<'de> {
    keys: Keys<'de, String, AttributeValue>,
    values: Values<'de, String, AttributeValue>,
}

impl<'de> AttributeValueMapDeserializer<'de> {
    pub fn new(values: &'de HashMap<String, AttributeValue>) -> Self {
        Self {
            keys: values.keys(),
            values: values.values(),
        }
    }
}

impl<'de> MapAccess<'de> for AttributeValueMapDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match self.keys.next() {
            Some(key) => seed
                .deserialize(AttributeValueMapKeyDeserializer::new(key))
                .map(Some),
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match self.values.next() {
            Some(value) => seed.deserialize(&mut AttributeValueDeserializer::new(value)),
            None => Err(Error::new("Value Expected")),
        }
    }
}

#[derive(new)]
struct AttributeValueMapKeyDeserializer<'de> {
    key: &'de str,
}

impl<'de> Deserializer<'de> for AttributeValueMapKeyDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.key.to_owned())
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct tuple_struct struct
        tuple enum identifier ignored_any
    }
}

// -----------------------------------------------------------------------------

// Attribute Value Seq Deserializer

use serde::de::SeqAccess;
use std::slice::Iter;

pub struct AttributeValueSeqDeserializer<'de> {
    values: Iter<'de, AttributeValue>,
}

impl<'de> AttributeValueSeqDeserializer<'de> {
    pub fn new(values: &'de [AttributeValue]) -> Self {
        Self {
            values: values.iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for AttributeValueSeqDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.values.next() {
            Some(value) => seed
                .deserialize(&mut AttributeValueDeserializer::new(value))
                .map(Some),
            None => Ok(None),
        }
    }
}

// -----------------------------------------------------------------------------

// Attribute Value Variant Deserializer

use serde::de::VariantAccess;

#[derive(new)]
pub struct AttributeValueVariantDeserializer<'de> {
    value: &'de AttributeValue,
}

impl<'de> VariantAccess<'de> for AttributeValueVariantDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        match self.value {
            AttributeValue {
                null: Some(true), ..
            } => Ok(()),
            _ => Err(Error::new("Null Value Expected")),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut AttributeValueDeserializer::new(self.value))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue { l: Some(l), .. } => {
                visitor.visit_seq(AttributeValueSeqDeserializer::new(l))
            }
            _ => Err(Error::new("List Value Expected")),
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            AttributeValue { m: Some(m), .. } => {
                visitor.visit_map(AttributeValueMapDeserializer::new(m))
            }
            _ => Err(Error::new("Map Value Expected")),
        }
    }
}

// =============================================================================

// Attribute Value Deserialization Functions

pub fn from_attribute_value<'a, T>(value: &'a AttributeValue) -> Result<T>
where
    T: Deserialize<'a>,
{
    T::deserialize(&mut AttributeValueDeserializer::new(value))
}
