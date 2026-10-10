#![allow(warnings)]

// Field names that match locals of the generated `DeserializeToolCallParam::from_str`
// (`key`, `value`, `src`, `json_iterator`, `next_item`, `result`) must not clash with them.

use json_schema::macros::ApplyJsonSchema;
use serde::{Deserialize, Serialize};

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct SecretHashModel {
    #[property(description = "key")]
    pub key: String,
    #[property(description = "value")]
    pub value: Option<String>,
}

#[derive(ApplyJsonSchema, Debug)]
pub struct AllMacroLocalsModel {
    #[property(description = "src")]
    pub src: String,
    #[property(description = "json_iterator")]
    pub json_iterator: Option<i64>,
    #[property(description = "next_item")]
    pub next_item: Vec<String>,
    #[property(description = "result")]
    pub result: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use json_schema::deserializer::impl_from_str::DeserializeToolCallParam;
    use json_schema::my_json;

    use super::{AllMacroLocalsModel, SecretHashModel};

    #[test]
    fn test_deserialize_key_and_value() {
        let json = r#"{"key": "my-key", "value": "my-value"}"#;

        let result = SecretHashModel::from_str(json).unwrap();

        assert_eq!(result.key, "my-key");
        assert_eq!(result.value.as_deref(), Some("my-value"));
    }

    #[test]
    fn test_deserialize_key_with_null_value() {
        let json = r#"{"key": "my-key", "value": null}"#;

        let result = SecretHashModel::from_str(json).unwrap();

        assert_eq!(result.key, "my-key");
        assert!(result.value.is_none());
    }

    #[test]
    fn test_deserialize_missing_key() {
        let json = r#"{"value": "my-value"}"#;

        let err = SecretHashModel::from_str(json).unwrap_err();

        assert_eq!(err, "Json field `key` is missing");
    }

    #[test]
    fn test_deserialize_all_macro_locals() {
        let json = r#"{"src": "my-src", "json_iterator": 5, "next_item": ["a", "b"], "result": ["c"]}"#;

        let result = AllMacroLocalsModel::from_str(json).unwrap();

        assert_eq!(result.src, "my-src");
        assert_eq!(result.json_iterator, Some(5));
        assert_eq!(result.next_item, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(result.result, Some(vec!["c".to_string()]));
    }

    #[tokio::test]
    async fn test_schema_generation() {
        let description = SecretHashModel::get_json_schema(false).await.build();

        let result = my_json::j_path::get_value(description.as_bytes(), "properties.key.type")
            .unwrap()
            .unwrap();

        assert_eq!(result.as_str().unwrap().as_str(), "string");

        let result =
            my_json::j_path::get_value(description.as_bytes(), "properties.value.description")
                .unwrap()
                .unwrap();

        assert_eq!(result.as_str().unwrap().as_str(), "value");

        let required: serde_json::Value = serde_json::from_str(description.as_str()).unwrap();

        assert_eq!(required["required"], serde_json::json!(["key"]));
    }
}
