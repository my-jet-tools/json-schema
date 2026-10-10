#![allow(warnings)]
use json_schema::macros::ApplyJsonSchema;

#[derive(ApplyJsonSchema)]
pub struct MyEnuModel {
    #[property(enum:["1","2","3"] description: "city")]
    pub my_enum: Option<String>,
}

#[cfg(test)]
mod test {
    use json_schema::JsonTypeDescription;

    use super::MyEnuModel;

    #[tokio::test]
    async fn test() {
        let data = MyEnuModel::get_json_schema(false).await;

        println!("{}", data.build());

        //let j: serde_json::Value = serde_json::from_str(data.build().as_str()).unwrap();

        //println!("{}", serde_json::to_string_pretty(&j).unwrap());
    }
}
