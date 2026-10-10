#![allow(warnings)]
use json_schema::macros::ApplyJsonSchema;

#[derive(ApplyJsonSchema)]
pub struct TestDeserializingValueOpt {
    #[property(description:"test")]
    pub items: Option<Vec<String>>,
}

#[cfg(test)]
mod test {
    use json_schema::deserializer::impl_from_str::DeserializeToolCallParam;

    #[test]
    fn test() {
        let json = r#"{ "items": "MyItem"}"#;

        let result = super::TestDeserializingValueOpt::from_str(json).unwrap();

        let items = result.items.unwrap();

        assert_eq!(items.len(), 1);

        assert_eq!(items.get(0).unwrap(), "MyItem");
    }
}
