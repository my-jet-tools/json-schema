#![allow(warnings)]
use json_schema::macros::ApplyJsonSchema;

#[derive(ApplyJsonSchema, Debug)]
pub struct NullValuesModel {
    #[property(description: "name")]
    pub name: String,
    #[property(description: "tags")]
    pub tags: Vec<String>,
    #[property(description: "note")]
    pub note: Option<String>,
}

#[cfg(test)]
mod tests {
    use json_schema::deserializer::impl_from_str::DeserializeToolCallParam;

    use super::NullValuesModel;

    #[test]
    fn test_null_for_required_field_is_error() {
        let err = NullValuesModel::from_str(r#"{"name": null, "tags": []}"#).unwrap_err();

        assert_eq!(err, "Value of `name` cannot be null");
    }

    #[test]
    fn test_null_for_required_array_is_error() {
        let err = NullValuesModel::from_str(r#"{"name": "a", "tags": null}"#).unwrap_err();

        assert_eq!(err, "Value of `tags` cannot be null");
    }

    #[test]
    fn test_null_for_optional_field_is_none() {
        let result = NullValuesModel::from_str(r#"{"name": "a", "tags": [], "note": null}"#).unwrap();

        assert!(result.note.is_none());
    }
}
