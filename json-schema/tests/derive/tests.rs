#![allow(warnings)]

use json_schema::macros::ApplyJsonSchema;
use rust_extensions::StrOrString;

#[derive(ApplyJsonSchema)]
pub struct MyRequestModel {
    #[property(description: "city")]
    pub city: String,
    #[property(description: "service")]
    pub service: Option<String>,
    #[property(description: "Address of dealer")]
    pub addr: Option<String>,
    #[property(enum:["NEW", "CPO"], description: "Vehicle condition (NEW/CPO). Defaults to None")]
    pub condition: Option<String>,

    #[property(enum: "get_other_condition_enum", description: "Vehicle condition (NEW/CPO). Defaults to None")]
    pub other_condition: String,

    #[property(description: "Minimal condition")]
    pub min_condition: Option<i64>,

    #[property(enum:["Value1", "Value2", "Value3"], description: "Several enum values")]
    pub several_enum: Vec<String>,

    #[property(enum:["Value1", "Value2", "Value3"], description: "Several enum values optional")]
    pub several_enum_opt: Option<Vec<String>>,
}

async fn get_other_condition_enum() -> Option<Vec<StrOrString<'static>>> {
    None
}

#[cfg(test)]
mod tests {
    use crate::tests::MyRequestModel;
    use json_schema::JsonTypeDescription;

    #[tokio::test]
    async fn test_generation() {
        let description = MyRequestModel::get_json_schema(false).await;

        println!("{}", description.build());
    }
}
