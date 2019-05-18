use rusoto_dynamodb::AttributeValue;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_rusoto_dynamodb::result::Error;
use std::{cmp::PartialEq, fmt::Debug};

// Helpers

fn roundtrip<T>(value: &T, expected: &AttributeValue)
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    match serde_rusoto_dynamodb::to_attribute_value(&value) {
        Ok(serialized) => {
            assert_eq!(&serialized, expected);
            match serde_rusoto_dynamodb::from_attribute_value::<T>(&serialized) {
                Ok(deserialized) => {
                    assert_eq!(&deserialized, value);
                }
                Err(Error { message }) => panic!("Serialization failed with message: {}", message),
            }
        }
        Err(Error { message }) => panic!("Serialization failed with message: {}", message),
    }
}

// Roundtrip

#[cfg(test)]
mod roundtrip {

    use super::*;
    use maplit::hashmap;

    // Boolean Values

    #[test]
    fn roundtrip_bool() {
        roundtrip(
            &true,
            &AttributeValue {
                bool: Some(true),
                ..AttributeValue::default()
            },
        );
        roundtrip(
            &false,
            &AttributeValue {
                bool: Some(false),
                ..AttributeValue::default()
            },
        );
    }

    // Numeric Values

    #[test]
    fn roundtrip_numeric() {
        roundtrip(
            &1i8,
            &AttributeValue {
                n: Some("1".to_owned()),
                ..AttributeValue::default()
            },
        );
        roundtrip(
            &1i16,
            &AttributeValue {
                n: Some("1".to_owned()),
                ..AttributeValue::default()
            },
        );
        roundtrip(
            &1i32,
            &AttributeValue {
                n: Some("1".to_owned()),
                ..AttributeValue::default()
            },
        );
        roundtrip(
            &1i64,
            &AttributeValue {
                n: Some("1".to_owned()),
                ..AttributeValue::default()
            },
        );

        roundtrip(
            &1u8,
            &AttributeValue {
                n: Some("1".to_owned()),
                ..AttributeValue::default()
            },
        );
        roundtrip(
            &1u16,
            &AttributeValue {
                n: Some("1".to_owned()),
                ..AttributeValue::default()
            },
        );
        roundtrip(
            &1u32,
            &AttributeValue {
                n: Some("1".to_owned()),
                ..AttributeValue::default()
            },
        );
        roundtrip(
            &1u64,
            &AttributeValue {
                n: Some("1".to_owned()),
                ..AttributeValue::default()
            },
        );

        roundtrip(
            &1.234f32,
            &AttributeValue {
                n: Some("1.234".to_owned()),
                ..AttributeValue::default()
            },
        );
        roundtrip(
            &2.345f64,
            &AttributeValue {
                n: Some("2.345".to_owned()),
                ..AttributeValue::default()
            },
        );
    }

    // Char Values

    #[test]
    fn serialize_character() {
        roundtrip(
            &'a',
            &AttributeValue {
                s: Some("a".to_owned()),
                ..AttributeValue::default()
            },
        );
    }

    // String Values

    #[test]
    fn serialize_string() {
        roundtrip(
            &"hello".to_owned(),
            &AttributeValue {
                s: Some("hello".to_owned()),
                ..AttributeValue::default()
            },
        );
    }

    // Option Values

    #[test]
    fn serialize_option() {
        roundtrip(
            &Some(true),
            &AttributeValue {
                bool: Some(true),
                ..AttributeValue::default()
            },
        );
        roundtrip(
            &None::<Option<bool>>,
            &AttributeValue {
                null: Some(true),
                ..AttributeValue::default()
            },
        );
    }

    // Seq Values

    #[test]
    fn serialize_seq() {
        roundtrip(
            &vec![1, 2, 3],
            &AttributeValue {
                l: Some(vec![
                    AttributeValue {
                        n: Some("1".to_owned()),
                        ..AttributeValue::default()
                    },
                    AttributeValue {
                        n: Some("2".to_owned()),
                        ..AttributeValue::default()
                    },
                    AttributeValue {
                        n: Some("3".to_owned()),
                        ..AttributeValue::default()
                    },
                ]),
                ..AttributeValue::default()
            },
        );
    }

    // Struct Values

    #[test]
    fn serialize_struct() {
        #[derive(Debug, Deserialize, PartialEq, Serialize)]
        struct Test {
            a: String,
            b: i32,
        }

        let test: Test = Test {
            a: "hello".to_owned(),
            b: 1,
        };

        roundtrip(
            &test,
            &AttributeValue {
                m: Some(hashmap! {
                    "a".to_owned() => AttributeValue {
                        s: Some("hello".to_owned()),
                        ..AttributeValue::default()
                    },
                    "b".to_owned() => AttributeValue {
                        n: Some("1".to_owned()),
                        ..AttributeValue::default()
                    }
                }),
                ..AttributeValue::default()
            },
        );
    }

    #[test]
    fn serialize_tuple() {
        roundtrip(
            &("hello".to_owned(), 37),
            &AttributeValue {
                l: Some(vec![
                    AttributeValue {
                        s: Some("hello".to_owned()),
                        ..AttributeValue::default()
                    },
                    AttributeValue {
                        n: Some("37".to_owned()),
                        ..AttributeValue::default()
                    },
                ]),
                ..AttributeValue::default()
            },
        )
    }

    #[test]
    fn serialize_unit() {
        roundtrip(
            &(),
            &AttributeValue {
                null: Some(true),
                ..AttributeValue::default()
            },
        )
    }

    // #[test]
    // fn serialize_unit_variant() {
    //     #[derive(Serialize)]
    //     enum Test {
    //         Unit,
    //     }

    //     let test: Test = Test::Unit;

    //     assert_eq!(
    //         serde_rusoto_dynamodb::to_attribute_value(test),
    //         Ok(av_s("Unit"))
    //     );
    // }
}
