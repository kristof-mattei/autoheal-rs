use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;
use std::str::FromStr;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_names<'de, D>(deserializer: D) -> Result<Rc<[String]>, D::Error>
where
    D: Deserializer<'de>,
{
    struct SeqVisitor();

    impl<'de> Visitor<'de> for SeqVisitor {
        type Value = Rc<[String]>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of items")
        }

        fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: SeqAccess<'de>,
        {
            let mut buffer = access.size_hint().map_or_else(Vec::new, Vec::with_capacity);

            while let Some(mut value) = access.next_element::<String>()? {
                // Docker container name starts with a '/'. I don't know why. But it's useless.
                if value.starts_with('/') {
                    let split = value.split_off(1);

                    buffer.push(split);
                } else {
                    buffer.push(value);
                }
            }

            Ok(buffer.into())
        }
    }

    let visitor = SeqVisitor();
    deserializer.deserialize_seq(visitor)
}

fn deserialize_timeout<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de> + FromStr,
    T::Err: std::fmt::Debug,
    D: Deserializer<'de>,
{
    struct MapVisitor<V>(PhantomData<fn() -> V>);

    impl<'de, V> Visitor<'de> for MapVisitor<V>
    where
        V: Deserialize<'de> + FromStr,
        V::Err: std::fmt::Debug,
    {
        type Value = Option<V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of items")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            while let Some((key, value)) = access.next_entry::<String, String>()? {
                if key == "autoheal.stop.timeout" {
                    let v = Some(value.parse::<V>().unwrap());
                    return Ok(v);
                }
            }

            Ok(None)
        }
    }

    let visitor = MapVisitor(PhantomData);
    deserializer.deserialize_map(visitor)
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub id: Rc<str>,
    #[serde(deserialize_with = "deserialize_names")]
    #[serde(rename(deserialize = "Names"))]
    pub names: Rc<[String]>,
    pub state: Rc<str>,
    #[serde(deserialize_with = "deserialize_timeout")]
    #[serde(rename(deserialize = "Labels"))]
    pub timeout: Option<u32>,
}

impl Container {
    pub fn get_name(&self) -> Option<String> {
        self.names.iter().cloned().reduce(|mut p, n| {
            p.push_str(&n);
            p
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::container::Container;

    #[test]
    fn test_deserialize() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism"],"Labels":{},"State":"running"},{"Id":"281ea0c72e2e4a41fd2f81df945da9dfbfbc7ea0fe5e59c3d2a8234552e367cf","Names":["/whoogle-search"],"Labels":{},"State":"running"}]"#;

        let deserialized: Result<Vec<Container>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[
                Container {
                    id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                    names: ["photoprism".into()].into(),
                    state: "running".into(),
                    timeout: None,
                },
                Container {
                    id: "281ea0c72e2e4a41fd2f81df945da9dfbfbc7ea0fe5e59c3d2a8234552e367cf".into(),
                    names: ["whoogle-search".into()].into(),
                    state: "running".into(),
                    timeout: None,
                }
            ] as &[Container],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_multiple_names() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism-1","/photoprism-2"],"Labels":{}, "State":"running"}]"#;

        let deserialized: Result<Vec<Container>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[Container {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                names: ["photoprism-1".into(), "photoprism-2".into()].into(),
                state: "running".into(),
                timeout: None,
            }][..],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_timeout() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism"],"State":"running","Labels":{"autoheal.stop.timeout":"12"}}]"#;

        let deserialized: Result<Vec<Container>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[Container {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                names: ["photoprism".into()].into(),
                state: "running".into(),
                timeout: Some(12),
            }][..],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_no_labels() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism"],"State":"running"}]"#;

        let deserialized: Result<Vec<Container>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_err());
    }

    #[test]
    fn test_deserialize_missing_timeout() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism"],"State":"running","Labels":{"autoheal.stop.other_label":"some_value"}}]"#;

        let deserialized: Result<Vec<Container>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[Container {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                names: ["photoprism".into()].into(),
                state: "running".into(),
                timeout: None,
            }][..],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_with_no_names_array() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","State":"running","Labels":{"autoheal.stop.other_label":"some_value"}}]"#;

        let deserialized: Result<Vec<Container>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_err());
    }

    #[test]
    fn test_deserialize_names_empty_names_array() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":[],"State":"running","Labels":{"autoheal.stop.other_label":"some_value"}}]"#;

        let deserialized: Result<Vec<Container>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[Container {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                names: vec![].into(),
                state: "running".into(),
                timeout: None,
            }][..],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_multiple_names_with_and_without_slash() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism-1","photoprism-2"],"Labels": {}, "State":"running"}]"#;

        let deserialized: Result<Vec<Container>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[Container {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                names: ["photoprism-1".into(), "photoprism-2".into()].into(),
                state: "running".into(),
                timeout: None,
            }][..],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_invalid_labels() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/foo"],"State":"running","Labels": "I am not a map, but a string"}]"#;

        let deserialized: Result<Vec<Container>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_err());

        assert_eq!(
            deserialized.unwrap_err().to_string(),
            "invalid type: string \"I am not a map, but a string\", expected a nonempty sequence of items at line 1 column 149"
        );
    }
}
