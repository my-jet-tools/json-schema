#![allow(warnings)]

// `FromStr` imported next to the struct brings a second `from_str` for `String` and `u32`
// into scope - the generated code must still pick `DeserializeToolCallParam::from_str`.

use json_schema::macros::ApplyJsonSchema;
use std::str::FromStr;

#[derive(ApplyJsonSchema)]
pub struct ModelNextToFromStr {
    #[property(description: "name")]
    pub name: String,
    #[property(description: "count")]
    pub count: Option<u32>,
}

#[cfg(test)]
mod tests {
    use json_schema::deserializer::impl_from_str::DeserializeToolCallParam;

    use super::ModelNextToFromStr;

    #[test]
    fn test_deserialize() {
        let result = ModelNextToFromStr::from_str(r#"{"name": "a", "count": 2}"#).unwrap();

        assert_eq!(result.name, "a");
        assert_eq!(result.count, Some(2));
    }
}
