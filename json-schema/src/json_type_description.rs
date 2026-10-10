use std::str::FromStr;

use my_json::json_writer::{JsonObjectWriter, JsonValueWriter};
use rust_extensions::StrOrString;

#[async_trait::async_trait]
pub trait JsonTypeDescription {
    async fn get_description(
        has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        output: bool,
    ) -> JsonObjectWriter;

    /// Renders a value given as text (an `enum` entry or a `default`) as JSON of this type,
    /// so a number field gets `5`, not `"5"`.
    fn schema_value(value: StrOrString<'static>) -> SchemaValue {
        SchemaValue::String(value)
    }
}

/// A value inside a schema (an `enum` entry or a `default`) in the JSON form of its field type.
pub enum SchemaValue {
    String(StrOrString<'static>),
    /// Valid JSON as is: a number or a boolean
    Raw(String),
}

impl JsonValueWriter for SchemaValue {
    const IS_ARRAY: bool = false;

    fn write(&self, dest: &mut String) {
        match self {
            Self::String(value) => value.write(dest),
            Self::Raw(value) => dest.push_str(value),
        }
    }
}

macro_rules! impl_json_type_description {
    ($json_type:literal, $schema_value:expr, [$($tp:ty),+]) => {
        $(
            #[async_trait::async_trait]
            impl JsonTypeDescription for $tp {
                async fn get_description(
                    _has_default: bool,
                    with_enum: Option<Vec<StrOrString<'static>>>,
                    _output: bool,
                ) -> JsonObjectWriter {
                    JsonObjectWriter::new().write("type", $json_type).write_iter_if_some(
                        "enum",
                        with_enum.map(|values| values.into_iter().map(Self::schema_value)),
                    )
                }

                fn schema_value(value: StrOrString<'static>) -> SchemaValue {
                    ($schema_value)(value)
                }
            }
        )+
    };
}

impl_json_type_description!(
    "integer",
    integer_value::<Self>,
    [u8, i8, u16, i16, u32, i32, u64, i64, usize, isize]
);
impl_json_type_description!("number", float_value, [f32, f64]);
impl_json_type_description!("boolean", bool_value, [bool]);
impl_json_type_description!("string", SchemaValue::String, [String, str]);

#[async_trait::async_trait]
impl<T: JsonTypeDescription + ?Sized> JsonTypeDescription for &T {
    async fn get_description(
        has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        output: bool,
    ) -> JsonObjectWriter {
        T::get_description(has_default, with_enum, output).await
    }

    fn schema_value(value: StrOrString<'static>) -> SchemaValue {
        T::schema_value(value)
    }
}

// A value which is not of the field type stays a string, as it was given.

fn integer_value<T: FromStr + ToString>(value: StrOrString<'static>) -> SchemaValue {
    match value.as_str().trim().parse::<T>() {
        Ok(number) => SchemaValue::Raw(number.to_string()),
        Err(_) => SchemaValue::String(value),
    }
}

fn float_value(value: StrOrString<'static>) -> SchemaValue {
    match value.as_str().trim().parse::<f64>() {
        // `NaN` and `inf` parse, but are not JSON numbers
        Ok(number) if number.is_finite() => SchemaValue::Raw(number.to_string()),
        _ => SchemaValue::String(value),
    }
}

fn bool_value(value: StrOrString<'static>) -> SchemaValue {
    let text = value.as_str().trim();

    if text.eq_ignore_ascii_case("true") || text.eq_ignore_ascii_case("false") {
        return SchemaValue::Raw(text.to_ascii_lowercase());
    }

    SchemaValue::String(value)
}
