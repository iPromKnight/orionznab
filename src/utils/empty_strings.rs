//! Deserializes empty strings as [None] into whatever type implementing [FromStr].
#[allow(dead_code)]
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serializer};

#[allow(dead_code)]
pub(crate) fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: serde::Serialize,
{
    serializer.serialize_some(value)
}

#[allow(dead_code)]
pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromStr,
    T::Err: std::fmt::Display,
{
    let value = Option::<String>::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(value) => {
            if value.is_empty() {
                Ok(None)
            } else {
                Ok(Some(T::from_str(&value).map_err(serde::de::Error::custom)?))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestingStruct<T>
    where
        T: ToString + for<'a> serde::Deserialize<'a> + serde::Serialize,
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        #[serde(with = "super")]
        value: Option<T>,
    }

    mod string {
        use super::TestingStruct;

        #[test]
        fn should_deserialize() {
            let result: TestingStruct<String> = serde_json::from_str(r#"{"value":null}"#).unwrap();
            assert_eq!(result.value, None);

            let result: TestingStruct<String> = serde_json::from_str(r#"{"value":""}"#).unwrap();
            assert_eq!(result.value, None);

            let result: TestingStruct<String> =
                serde_json::from_str(r#"{"value":"test"}"#).unwrap();
            assert_eq!(result.value, Some("test".to_owned()));
        }

        #[test]
        fn should_serialize() {
            let result = serde_json::to_string(&TestingStruct::<String> { value: None }).unwrap();
            assert_eq!(result, r#"{"value":null}"#);

            let result = serde_json::to_string(&TestingStruct::<String> {
                value: Some("test".to_owned()),
            })
                .unwrap();
            assert_eq!(result, r#"{"value":"test"}"#);
        }
    }
}