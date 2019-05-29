// Serialization

// Serialize Rust values to the native AWS AttributeValue representation used
// by DynamoDB (and related products such as Streams) and which is implemented
// in Rust by the Rusoto family of libraries.

use super::result::{Error, Result};
use maplit::hashmap;
use rusoto_dynamodb::AttributeValue;
use serde::ser::{Serialize, Serializer};

// Attribute Value Serializer

// A relatively simple custom Serializer for converting Serde-compatible types
// to the Rusoto AttributeValue form, with simple conventions largely based on
// JSON representations of Rust types (though with some exceptions, documented
// where relevant).

#[derive(new)]
struct AttributeValueSerializer;

use itoa::Integer;
use ryu::{Buffer, Float};

impl AttributeValueSerializer {
    // Numeric

    // Implementations of numeric value serializtion helper functions - made
    // available as generic functions to unify serialization logic for the two
    // numeric families (float and int).

    // Fast serialization libraries (itoa and ryu) are used to provide the
    // underlying implementation of a string formatted number, as the AWS
    // AttributeValue representation of a number is a string value.

    fn serialize_float<F: Float>(&self, v: F) -> Result<AttributeValue> {
        let mut buf = Buffer::new();

        Ok(AttributeValue {
            n: Some(buf.format(v).to_owned()),
            ..AttributeValue::default()
        })
    }

    fn serialize_int<I: Integer>(&self, v: I) -> Result<AttributeValue> {
        let mut s = String::new();
        itoa::fmt(&mut s, v).unwrap();

        Ok(AttributeValue {
            n: Some(s.to_owned()),
            ..AttributeValue::default()
        })
    }
}

impl Serializer for AttributeValueSerializer {
    // Return Types

    // Use the Rusoto AttributeValue as our primary "Ok" type and the custom
    // Error type defined at the top level of the library for our "Error" type.

    type Ok = AttributeValue;
    type Error = Error;

    // Serializers

    // Use a custom compound serializer for each of the type variables relevant
    // to Rust value serialization, implemented below.

    type SerializeMap = AttributeValueMapSerializer;
    type SerializeSeq = AttributeValueSeqTupleAndTupleStructSerializer;
    type SerializeStruct = AttributeValueStructSerializer;
    type SerializeStructVariant = AttributeValueStructVariantSerializer;
    type SerializeTuple = AttributeValueSeqTupleAndTupleStructSerializer;
    type SerializeTupleStruct = AttributeValueSeqTupleAndTupleStructSerializer;
    type SerializeTupleVariant = AttributeValueTupleVariantSerializer;

    // Boolean

    // Serialize boolean values using the native bool representation of the
    // AWS AttributeValue type.

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(AttributeValue {
            bool: Some(v),
            ..AttributeValue::default()
        })
    }

    // Numeric

    // Serialize numeric (float and int) values using the native number
    // representation of the AWS AttributeValue type (note that the data is
    // stored as a string, see the implementation of string serializers above).

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_int(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_int(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_int(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.serialize_int(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_int(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_int(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_int(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.serialize_int(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.serialize_float(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.serialize_float(v)
    }

    // Character

    // Serialize character values as a string using the native string
    // representation of the AWS AttributeValue type (a string of length 1).

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        Ok(AttributeValue {
            s: Some(value.to_string()),
            ..AttributeValue::default()
        })
    }

    // String

    // Serialize string values as a string using the native string
    // representation of the AWS AttributeValue type.

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        Ok(AttributeValue {
            s: Some(value.to_string()),
            ..AttributeValue::default()
        })
    }

    // Bytes

    // Serialize byte slice values as a string using the native byte vector
    // representation of the AWS AttributeValue type.

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        Ok(AttributeValue {
            b: Some(value.to_vec()),
            ..AttributeValue::default()
        })
    }

    // Map

    // Serialize map values using the compound serializer defined by the type
    // variable for SerializeMap (see the implementation later).

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Default::default())
    }

    // Option

    // Serialize Option values by representing None as the native null
    // representation of the AWS AttributeValue type, and Some as the serialized
    // form of the inner value.

    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_some<V: ?Sized>(self, value: &V) -> Result<Self::Ok>
    where
        V: Serialize,
    {
        value.serialize(AttributeValueSerializer)
    }

    // Newtype

    // Serialize the newtype forms in appropriate ways:

    // For newtype structs, represent as the serialized form of the newtype
    // value.

    // For newtype variants, follow the library approach to variants of a single
    // key/value  in a map (using the native AWS AttributeValue map
    // representation) where the key represents the variant name and the value
    // the serialized form of the variant value (in this case, the serialized
    // newtype form).

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(AttributeValueSerializer)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Ok(AttributeValue {
            m: Some(hashmap! {
                variant.to_owned() => value.serialize(AttributeValueSerializer)?
            }),
            ..AttributeValue::default()
        })
    }

    // Seq

    // Serialize seq values using the compound serializer defined by the type
    // variable for SerializeSeq (see the implementation later).

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(Default::default())
    }

    // Struct

    // Serialize the struct forms in appropriate ways:

    // For the basic struct, using the compound serializer defined by the type
    // variable for SerializeStruct (see the implementation later).

    // For the struct variant, using the compound serializer defined by the type
    // variable for SerializeStructVariant (see the implementation later).

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(Default::default())
    }

    fn serialize_struct_variant(
        self,
        _enum: &'static str,
        _idx: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(AttributeValueStructVariantSerializer::new(
            variant.to_owned(),
        ))
    }

    // Tuple

    // Serialize the tuple forms in appropriate ways:

    // For the basic tuple, using the compound serializer defined by the type
    // variable for SerializeTuple (see the implementation later).

    // For the tuple struct, using the compound serializer defined by the type
    // variable for SerializeTupleStruct (see the implementation later).

    // For the tuple variant, using the compound serializer defined by the type
    // variable for SerializeTupleVariant(see the implementation later).

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(Default::default())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(Default::default())
    }

    fn serialize_tuple_variant(
        self,
        _enum: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(AttributeValueTupleVariantSerializer::new(
            variant.to_owned(),
        ))
    }

    // Unit

    // Serialize the unit forms in appropriate ways:

    // For unit and the unit struct, serialize as null using the native null
    // representation of the AWS AttributeValue type.

    // For the unit variant, serialize using the map form as described in the
    // serialization of the newtype variant, where the value will be the native
    // AWS AttributeValue representation of null. This differs from common
    // serialization approaches where the unit variant is stored as a string
    // containing the variant name, but this approach is more consistent with
    // the other variant forms.

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(AttributeValue {
            null: Some(true),
            ..AttributeValue::default()
        })
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(AttributeValue {
            m: Some(hashmap! {
                variant.to_owned() => AttributeValue {
                    null: Some(true),
                    ..AttributeValue::default()
                }
            }),
            ..AttributeValue::default()
        })
    }
}

// =============================================================================

// Compound Serializers

use std::collections::HashMap;

// -----------------------------------------------------------------------------

// Attribute Value Map Serializer

// Serialize Rust map values as the native AWS AttributeValue map type. Keys
// must be strings, so we serialize the key values and reject non-string results
// and use valid keys when inserting the next value in to the HashMap.

use serde::ser::SerializeMap;

#[derive(Default)]
pub struct AttributeValueMapSerializer {
    key: Option<String>,
    values: HashMap<String, AttributeValue>,
}

impl SerializeMap for AttributeValueMapSerializer {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        match key.serialize(AttributeValueSerializer) {
            Ok(AttributeValue { s: Some(s), .. }) => {
                self.key = Some(s);
                Ok(())
            }
            _ => Err(Error::new("Key Must Be String")),
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        match (
            self.key.to_owned(),
            value.serialize(AttributeValueSerializer),
        ) {
            (Some(s), Ok(value)) => {
                self.values.insert(s.to_owned(), value);
                Ok(())
            }
            _ => Err(Error::new("Key Must Be Set and Value Must Be Serializable")),
        }
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(AttributeValue {
            m: Some(self.values),
            ..AttributeValue::default()
        })
    }
}

// -----------------------------------------------------------------------------

// Attribute Value Seq, Tuple and Tuple Struct Serializer

// Serialize Rust seq values as the native AWS AttributeValue list type where
// each element is serialized as an AttributeValue (making a homogenous list
// even though we also use this approach here for tuples which would otherwise
// imply heteregenous lists).

use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct};

#[derive(Default)]
pub struct AttributeValueSeqTupleAndTupleStructSerializer {
    values: Vec<AttributeValue>,
}

impl AttributeValueSeqTupleAndTupleStructSerializer {
    fn serialize<T: ?Sized>(&mut self, elem: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.values.push(elem.serialize(AttributeValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<AttributeValue> {
        Ok(AttributeValue {
            l: Some(self.values),
            ..AttributeValue::default()
        })
    }
}

impl SerializeSeq for AttributeValueSeqTupleAndTupleStructSerializer {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, elem: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.serialize(elem)
    }

    fn end(self) -> Result<Self::Ok> {
        self.end()
    }
}

impl SerializeTuple for AttributeValueSeqTupleAndTupleStructSerializer {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, elem: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.serialize(elem)
    }

    fn end(self) -> Result<AttributeValue> {
        self.end()
    }
}

impl SerializeTupleStruct for AttributeValueSeqTupleAndTupleStructSerializer {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_field<V: ?Sized>(&mut self, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        self.serialize(value)
    }

    fn end(self) -> Result<AttributeValue> {
        self.end()
    }
}

// -----------------------------------------------------------------------------

// Attribute Value Struct Serializer

// Serialize Rust struct values as the native AWS ATtributeValue map type, as
// struct keys are already compatible with the requirement for a string-keyed
// HashMap as the underlying store.

use serde::ser::SerializeStruct;

#[derive(Default)]
pub struct AttributeValueStructSerializer {
    values: HashMap<String, AttributeValue>,
}

impl SerializeStruct for AttributeValueStructSerializer {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_field<V: ?Sized>(&mut self, key: &'static str, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        self.values
            .insert(key.to_owned(), value.serialize(AttributeValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<AttributeValue> {
        Ok(AttributeValue {
            m: Some(self.values),
            ..AttributeValue::default()
        })
    }
}

// -----------------------------------------------------------------------------

// Attribute Value Struct Variant Serializer

// Serialize Rust struct variant values using a singly-keyed map containing the
// actual serialized variant data, where the key is the variant name. This maps
// to the same convention used by the previously defined newtype variant.

use serde::ser::SerializeStructVariant;

#[derive(new)]
pub struct AttributeValueStructVariantSerializer {
    #[new(default)]
    values: HashMap<String, AttributeValue>,
    variant: String,
}

impl SerializeStructVariant for AttributeValueStructVariantSerializer {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_field<V: ?Sized>(&mut self, field: &'static str, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        self.values
            .insert(field.to_owned(), value.serialize(AttributeValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<AttributeValue> {
        Ok(AttributeValue {
            m: Some(hashmap! {
                self.variant => AttributeValue {
                    m: Some(self.values),
                    ..AttributeValue::default()
                }
            }),
            ..AttributeValue::default()
        })
    }
}

// -----------------------------------------------------------------------------

// Attribute Value Tuple Variant Serializer

// Serialize Rust tuple variant values using a singly-keyed map containing the
// actual serialized variant data, where the key is the variant name. This maps
// to the same convention used by the previously defined newtype variant.

use serde::ser::SerializeTupleVariant;

#[derive(new)]
pub struct AttributeValueTupleVariantSerializer {
    #[new(default)]
    values: Vec<AttributeValue>,
    variant: String,
}

impl SerializeTupleVariant for AttributeValueTupleVariantSerializer {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_field<V: ?Sized>(&mut self, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        self.values.push(value.serialize(AttributeValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<AttributeValue> {
        Ok(AttributeValue {
            m: Some(hashmap! {
                self.variant => AttributeValue {
                    l: Some(self.values),
                    ..AttributeValue::default()
                }
            }),
            ..AttributeValue::default()
        })
    }
}

// =============================================================================

// Attribute Value Serialization Functions

pub fn to_attribute_value<T>(value: T) -> Result<AttributeValue>
where
    T: Serialize,
{
    value.serialize(AttributeValueSerializer::new())
}
