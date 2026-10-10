use my_json::json_writer::{JsonNullValue, JsonObjectWriter};
use rust_extensions::StrOrString;

use super::*;

#[async_trait::async_trait]
impl<T: JsonTypeDescription> JsonTypeDescription for Option<T> {
    async fn get_description(
        has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        output: bool,
    ) -> JsonObjectWriter {
        let tp = T::get_description(false, with_enum, output).await;

        // An input schema keeps the flat OpenAPI-style `nullable` flag. An output schema is what
        // MCP clients validate structured content against, so it is spelled in JSON Schema terms.
        if !output {
            return tp
                .write("nullable", true)
                .write_if("default", JsonNullValue, !has_default);
        }

        JsonObjectWriter::new()
            .write_json_array("anyOf", |arr| {
                arr.write_json_object(|o| o.write("type", "null"))
                    .write_ref(&tp)
            })
            .write_if("default", JsonNullValue, !has_default)
    }

    fn schema_value(value: StrOrString<'static>) -> SchemaValue {
        T::schema_value(value)
    }
}
