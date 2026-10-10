#![allow(warnings)]

use json_schema::macros::ApplyJsonSchema;
use json_schema::my_json::json_writer::EmptyJsonArray;
use rust_extensions::StrOrString;

#[derive(ApplyJsonSchema)]
pub struct MyRequestModelNoRequired {
    #[property(description: "city description")]
    pub city: Option<String>,
    #[property(description: "service")]
    pub service: Option<String>,
}

#[cfg(test)]
mod tests {
    use json_schema::{*, my_json};

    use crate::model_with_no_required::MyRequestModelNoRequired;

    #[tokio::test]
    async fn test_generation() {
        let description = MyRequestModelNoRequired::get_json_schema(false)
            .await
            .build();

        println!("{}", description.as_str());

        let result = my_json::j_path::get_value(description.as_bytes(), "type")
            .unwrap()
            .unwrap();

        assert_eq!(result.as_str().unwrap().as_str(), "object");

        let result = my_json::j_path::get_value(description.as_bytes(), "properties.city.type")
            .unwrap()
            .unwrap();

        assert_eq!(result.as_str().unwrap().as_str(), "string");

        let result =
            my_json::j_path::get_value(description.as_bytes(), "properties.city.description")
                .unwrap()
                .unwrap();

        assert_eq!(result.as_str().unwrap().as_str(), "city description");

        let result = my_json::j_path::get_value(description.as_bytes(), "properties.city.default")
            .unwrap()
            .unwrap();

        assert!(result.as_str().is_none());
    }
}
