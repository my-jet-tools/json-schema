use my_json::json_writer::JsonObjectWriter;

use super::*;

#[async_trait::async_trait]
impl<Tp: JsonTypeDescription> JsonTypeDescription for Vec<Tp> {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<rust_extensions::StrOrString<'static>>>,
        output: bool,
    ) -> JsonObjectWriter {
        // `true`: an item has no default of its own, so an `Option` item must not get `"default": null`
        let items = Tp::get_description(true, with_enum, output).await;
        JsonObjectWriter::new()
            .write("type", "array")
            .write("items", items)
    }

    fn schema_value(value: rust_extensions::StrOrString<'static>) -> SchemaValue {
        Tp::schema_value(value)
    }
}
