use std::{fmt, marker::PhantomData, str::FromStr};

use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

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

fn deserialize_first<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de> + Ord,
    D: Deserializer<'de>,
{
    struct MaxVisitor<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for MaxVisitor<T>
    where
        T: Deserialize<'de> + Ord,
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of items")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            if let Some(first) = seq.next_element()? {
                // SeqAccess requires us to visit all elements
                while (seq.next_element::<T>()?).is_some() {}

                Ok(first)
            } else {
                Ok(None)
            }
        }
    }

    let visitor = MaxVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    pub id: String,
    #[serde(deserialize_with = "deserialize_first")]
    #[serde(rename(deserialize = "Names"), default)]
    pub name: Option<String>,
    pub state: String,
    #[serde(deserialize_with = "deserialize_timeout")]
    #[serde(rename(deserialize = "Labels"), default)]
    pub timeout: Option<u32>,
}

#[cfg(test)]
mod tests {
    use crate::container_info::ContainerInfo;

    #[test]
    fn test_deserialize() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism"],"State":"running"},{"Id":"281ea0c72e2e4a41fd2f81df945da9dfbfbc7ea0fe5e59c3d2a8234552e367cf","Names":["/whoogle-search"],"State":"running"}]"#;

        let deserialized: Result<Vec<ContainerInfo>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[
                ContainerInfo {
                    id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                    name: Some("/photoprism".into()),
                    state: "running".into(),
                    timeout: None,
                },
                ContainerInfo {
                    id: "281ea0c72e2e4a41fd2f81df945da9dfbfbc7ea0fe5e59c3d2a8234552e367cf".into(),
                    name: Some("/whoogle-search".into()),
                    state: "running".into(),
                    timeout: None,
                }
            ] as &[ContainerInfo],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_multiple_names() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism","/photoprism-name-2"],"State":"running"}]"#;

        let deserialized: Result<Vec<ContainerInfo>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[ContainerInfo {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                name: Some("/photoprism".into()),
                state: "running".into(),
                timeout: None,
            }] as &[ContainerInfo],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_timeout() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism"],"State":"running","Labels":{"autoheal.stop.timeout":"12"}}]"#;

        let deserialized: Result<Vec<ContainerInfo>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[ContainerInfo {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                name: Some("/photoprism".into()),
                state: "running".into(),
                timeout: Some(12),
            }] as &[ContainerInfo],
            deserialized.unwrap()
        );
    }
    #[test]
    fn test_deserialize_no_labels() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism"],"State":"running"}]"#;

        let deserialized: Result<Vec<ContainerInfo>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[ContainerInfo {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                name: Some("/photoprism".into()),
                state: "running".into(),
                timeout: None,
            }] as &[ContainerInfo],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_missing_timeout() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":["/photoprism"],"State":"running","Labels":{"autoheal.stop.other_label":"some_value"}}]"#;

        let deserialized: Result<Vec<ContainerInfo>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[ContainerInfo {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                name: Some("/photoprism".into()),
                state: "running".into(),
                timeout: None,
            }] as &[ContainerInfo],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_with_no_name_array() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","State":"running","Labels":{"autoheal.stop.other_label":"some_value"}}]"#;

        let deserialized: Result<Vec<ContainerInfo>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[ContainerInfo {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                name: None,
                state: "running".into(),
                timeout: None,
            }] as &[ContainerInfo],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_name_array_with_1_null() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":[null],"State":"running","Labels":{"autoheal.stop.other_label":"some_value"}}]"#;

        let deserialized: Result<Vec<ContainerInfo>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[ContainerInfo {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                name: None,
                state: "running".into(),
                timeout: None,
            }] as &[ContainerInfo],
            deserialized.unwrap()
        );
    }

    #[test]
    fn test_deserialize_name_empty_name_array() {
        let input = r#"[{"Id":"582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae","Names":[],"State":"running","Labels":{"autoheal.stop.other_label":"some_value"}}]"#;

        let deserialized: Result<Vec<ContainerInfo>, _> = serde_json::from_reader(input.as_bytes());

        assert!(deserialized.is_ok());

        assert_eq!(
            &[ContainerInfo {
                id: "582036c7a5e8719bbbc9476e4216bfaf4fd318b61723f41f2e8fe3b60d8182ae".into(),
                name: None,
                state: "running".into(),
                timeout: None,
            }] as &[ContainerInfo],
            deserialized.unwrap()
        );
    }
}
