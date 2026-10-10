#![allow(warnings)]
use json_schema::macros::ApplyJsonSchema;

#[derive(ApplyJsonSchema)]
pub struct TypedValuesModel {
    #[property(description: "limit", default: "5")]
    pub limit: Option<u32>,
    #[property(enum: ["1", "2"], description: "level")]
    pub level: u8,
    #[property(description: "country", default: "bg")]
    pub country: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::TypedValuesModel;

    #[tokio::test]
    async fn test_values_have_the_field_type() {
        let schema = TypedValuesModel::get_json_schema(false).await.build();
        let schema: serde_json::Value = serde_json::from_str(schema.as_str()).unwrap();

        assert_eq!(
            schema["properties"]["limit"]["default"],
            serde_json::json!(5)
        );
        assert_eq!(
            schema["properties"]["level"]["enum"],
            serde_json::json!([1, 2])
        );
        assert_eq!(
            schema["properties"]["country"]["default"],
            serde_json::json!("bg")
        );
    }
}
