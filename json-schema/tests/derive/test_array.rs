#![allow(warnings)]
use json_schema::macros::ApplyJsonSchema;

#[derive(ApplyJsonSchema)]
pub struct MyRequestModel {
    #[property(description: "city")]
    pub city: String,
    #[property(description: "service")]
    pub service: Option<String>,

    #[property(description: "country", default:"bg")]
    pub country: Option<String>,
}

#[derive(ApplyJsonSchema)]
pub struct RootObject {
    #[property(description: "Items of array")]
    pub items: Vec<MyRequestModel>,
}

#[cfg(test)]
mod tests {
    use json_schema::JsonTypeDescription;

    use super::*;

    #[tokio::test]
    async fn test_array_generation() {
        let description = RootObject::get_json_schema(false).await;
        let json = description.build();

        println!("{}", json);
    }
}
