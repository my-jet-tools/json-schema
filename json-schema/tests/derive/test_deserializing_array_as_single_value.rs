#![allow(warnings)]
use json_schema::macros::ApplyJsonSchema;

#[derive(ApplyJsonSchema)]
pub struct TestDeserializingValue {
    #[property(description:"test")]
    pub items: Vec<String>,
}

#[cfg(test)]
mod test {
    use json_schema::deserializer::impl_from_str::DeserializeToolCallParam;

    use crate::test_deserializing_array_as_single_value::TestDeserializingValue;

    #[test]
    fn test() {
        let json = r#"{ "items": "MyItem"}"#;

        let result = TestDeserializingValue::from_str(json).unwrap();

        assert_eq!(result.items.len(), 1);

        assert_eq!(result.items.get(0).unwrap(), "MyItem");
    }
}
