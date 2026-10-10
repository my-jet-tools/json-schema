# json-schema

JSON Schema for LLM tool calls, generated from Rust structs, and a forgiving parser for the arguments a model sends back.

You describe the input (or output) of a tool as a struct, derive `ApplyJsonSchema`, and get:

- the JSON Schema for an MCP `inputSchema` / `outputSchema` or for the `tools` of a chat-completion request;
- a parser that turns the JSON arguments of the model back into the struct.

Schemas are built **when you ask for them, and asynchronously**. An `enum` can be filled by an async function, so the schema a model sees lists what is in the database right now, not what was there when the binary was built.

## Installation

```toml
[dependencies]
json-schema = { tag = "${last_tag}", git = "https://github.com/my-jet-tools/json-schema.git" }
```

This is the only dependency you need. The derive comes as `json_schema::macros::ApplyJsonSchema`, and the code it generates reaches `async-trait`, `my-json` and `rust-extensions` through `json_schema`. Keep the dependency name `json-schema`: the generated code refers to `::json_schema`.

## Quick start

```rust
use json_schema::macros::ApplyJsonSchema;
use json_schema::rust_extensions::StrOrString;

#[derive(ApplyJsonSchema)]
pub struct SearchCarsInput {
    #[property(description: "City to search in")]
    pub city: String,

    #[property(enum: "get_brands", description: "Car brand")]
    pub brand: Option<String>,

    #[property(enum: ["NEW", "CPO"], description: "Vehicle condition")]
    pub condition: Option<String>,

    #[property(description: "Max number of results", default: "20")]
    pub limit: Option<u32>,

    #[property(description: "Tags to match")]
    pub tags: Vec<String>,
}

// Called every time the schema is built
async fn get_brands() -> Option<Vec<StrOrString<'static>>> {
    let brands = vec!["BMW".to_string(), "Audi".to_string()]; // e.g. SELECT DISTINCT brand FROM cars
    Some(brands.into_iter().map(|brand| brand.into()).collect())
}
```

Build the schema:

```rust
let schema: String = SearchCarsInput::get_json_schema(false).await.build();
```

`false` asks for an input schema, `true` for an output schema (see [Input and output schemas](#input-and-output-schemas)). The result:

```json
{
  "type": "object",
  "properties": {
    "city": { "type": "string", "description": "City to search in" },
    "brand": {
      "type": "string",
      "enum": ["BMW", "Audi"],
      "nullable": true,
      "default": null,
      "description": "Car brand"
    },
    "condition": {
      "type": "string",
      "enum": ["NEW", "CPO"],
      "nullable": true,
      "default": null,
      "description": "Vehicle condition"
    },
    "limit": { "type": "integer", "nullable": true, "description": "Max number of results", "default": 20 },
    "tags": { "type": "array", "items": { "type": "string" }, "description": "Tags to match" }
  },
  "required": ["city", "tags"]
}
```

Parse what the model sent:

```rust
use json_schema::deserializer::impl_from_str::DeserializeToolCallParam;

let input = SearchCarsInput::from_str(r#"{"city": "Sofia", "tags": "family"}"#)?;

assert_eq!(input.tags, vec!["family"]); // a single value is taken as an array of one item
assert_eq!(input.limit, None);          // `default` is for the model; apply it yourself
```

## What the derive generates

`#[derive(ApplyJsonSchema)]` on a struct with named fields generates:

| Item | Purpose |
|---|---|
| `async fn get_json_schema(output: bool) -> JsonObjectWriter` | The schema of the struct. Call `.build()` to get a `String` |
| `impl JsonTypeDescription` | Lets the struct be a field of another derived struct: a nested object, `Vec<T>`, `Option<T>` |
| `impl DeserializeToolCallParam` | `from_str(&str) -> Result<Self, String>`, the parser of model output |

Every field needs a `#[property(...)]` attribute, otherwise the derive fails with `Attribute property not found`:

| Parameter | Meaning |
|---|---|
| `description: "…"` | Required. The `description` of the property |
| `enum: ["a", "b"]` | Allowed values, fixed at compile time |
| `enum: "fn_name"` | Allowed values from `async fn fn_name() -> Option<Vec<StrOrString<'static>>>`, called on every schema build. A path works too: `"crate::db::get_brands"`. `None` leaves the field without `enum` |
| `default: "…"` | The `default` of the property |

Values of `enum` and `default` are written in the type of the field: `default: "20"` on a `u32` field gives `"default": 20`, and `enum: ["1", "2"]` on a `u8` field gives `"enum": [1, 2]`. A value that does not parse as the field type stays a string. On a `Vec<T>` field, `enum` applies to the items.

Fields that are not `Option<T>` go to `required`.

## Type mapping

| Rust | Schema |
|---|---|
| `u8` … `u64`, `i8` … `i64`, `usize`, `isize` | `{"type": "integer"}` |
| `f32`, `f64` | `{"type": "number"}` |
| `bool` | `{"type": "boolean"}` |
| `String`, `&str` | `{"type": "string"}` |
| `Vec<T>` | `{"type": "array", "items": <schema of T>}` |
| `Option<T>` | The schema of `T` that also allows `null`, see below |
| A struct with `#[derive(ApplyJsonSchema)]` | `{"type": "object", "properties": {…}, "required": […]}` |
| Your own type | Implement the traits, see [Your own types](#your-own-types) |

`&str` can be described, but can not be a field of a derived struct: the parser needs an owned value.

## Input and output schemas

`get_json_schema(output)` differs only in how `Option<T>` is written.

Input schema, `get_json_schema(false)`:

```json
"next_page": { "type": "string", "nullable": true, "default": null, "description": "Cursor of the next page" }
```

Output schema, `get_json_schema(true)`:

```json
"next_page": {
  "anyOf": [{ "type": "null" }, { "type": "string" }],
  "default": null,
  "description": "Cursor of the next page"
}
```

An input schema is read by a model, so it keeps the flat form with `nullable`, as in OpenAPI. An output schema is checked by validators: MCP clients are expected to validate the `structuredContent` of a tool against its `outputSchema`. `nullable` is not a JSON Schema keyword, so a validator would ignore it and reject a `null`.

An `Option<T>` field without its own `default` gets `"default": null`: when the model leaves the field out, the tool gets `None`.

## Parsing model output

`DeserializeToolCallParam` is a separate parser, not serde. It forgives what models often get wrong and reports the rest with a message the model can act on:

| The model sends | Result |
|---|---|
| A field the struct does not have | Ignored |
| Nothing or `null` for an `Option<T>` field | `None` |
| Nothing for a field that is not `Option<T>` | ``Err("Json field `city` is missing")`` |
| `null` for a field that is not `Option<T>` | ``Err("Value of `city` cannot be null")`` |
| A single value for a `Vec<T>` field | An array of one item |
| A broken array | An error |
| A JSON string | The string unescaped: `\n`, `\"`, `\uXXXX` |
| A number or `true` for a `String` field | Its text: `42` becomes `"42"` |
| A number in quotes (`"5"`) for a number field | An error |

A struct can derive serde traits as well. `DeserializeToolCallParam` does not read `#[serde(...)]` attributes.

## Your own types

To use your own type as a field of a derived struct, implement `JsonTypeDescription` for the schema and `DeserializeToolCallParam` for the parser:

```rust
use json_schema::JsonTypeDescription;
use json_schema::deserializer::impl_from_str::DeserializeToolCallParam;
use json_schema::my_json::json_writer::JsonObjectWriter;
use json_schema::rust_extensions::StrOrString;

pub struct Email(pub String);

#[json_schema::async_trait::async_trait]
impl JsonTypeDescription for Email {
    async fn get_description(
        _has_default: bool,
        _with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> JsonObjectWriter {
        JsonObjectWriter::new()
            .write("type", "string")
            .write("format", "email")
    }
}

impl DeserializeToolCallParam for Email {
    fn from_str(src: &str) -> Result<Self, String> {
        let value = <String as DeserializeToolCallParam>::from_str(src)?;

        if !value.contains('@') {
            return Err(format!("`{value}` is not an email"));
        }

        Ok(Self(value))
    }
}
```

The parameters of `get_description`:

| Parameter | Meaning |
|---|---|
| `has_default` | The field has its own `default`, so `Option<T>` does not write `"default": null` |
| `with_enum` | The `enum` values of the field, if it has them |
| `output` | `true` for an output schema, `false` for an input schema |

`JsonTypeDescription::schema_value` turns an `enum` or `default` value into the JSON form of the type. By default the value is written as a string; override it if your type is a number or a boolean in JSON.

## Dynamic enums in an MCP server

- `get_json_schema` reads the values every time it is called, so call it when you answer `tools/list`, not once at startup.
- Clients keep the tool list they got. When the values change, send `notifications/tools/list_changed` (`notify_tools_changed()` in mcp-server-middleware), or the model keeps seeing the old values.
- The parser does not check that a value is in `enum`: a value that has just been removed from the database still reaches the tool. Check it there.

## Reusing the derive under another path

A crate that re-exports `json-schema` can offer the derive under its own path. Re-export the whole crate and call the code generator from your proc-macro crate with the path your users see:

```rust
// your-crate/src/lib.rs
pub use json_schema;

// your-crate-macros/src/lib.rs
#[proc_macro_derive(ApplyJsonSchema, attributes(property))]
pub fn apply_json_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    match json_schema_macros_core::generate(&ast, &quote::quote!(::your_crate::json_schema)) {
        Ok(result) => result.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
```

The generated code uses `JsonTypeDescription`, `deserializer`, `async_trait`, `my_json` and `rust_extensions` under that path, so the path has to lead to the whole crate. Your macro crate needs `syn` 3, the version `types-reader` is built on.

## Why not schemars

- Schemas here are built when you ask for them, asynchronously, so they can include values loaded from a database. schemars builds a schema from the type alone, synchronously.
- The output is shaped for tool calls: the flat `nullable` form for models, `anyOf` for output that gets validated, `enum` and `default` from attributes.
- It comes with a parser for model output that accepts a single value instead of an array, or a number where a string is expected. serde does not do that out of the box.
- It is built on `my-json` and `rust-extensions`, like the rest of the MyJetTools stack.

## Crates

| Crate | Contents |
|---|---|
| `json-schema` | The traits, their implementations for std types, the parser. Re-exports the derive as `json_schema::macros` |
| `json-schema-macros` | The `ApplyJsonSchema` derive |
| `json-schema-macros-core` | The code generator behind the derive, for use in another proc-macro crate |

## Limitations

- The derive takes structs with named fields and without generic parameters.
- `HashMap` / `BTreeMap`, `Box<T>`, tuples and Rust enums are not supported out of the box: implement the traits yourself.
- Property names are the Rust field names; `#[serde(rename)]` is not taken into account.
- `default` only describes the field. A field that is not `Option<T>` stays in `required`, and the parser does not fill defaults in.
- A number in quotes (`"5"`) for a number field is an error.
- The parser does not support `Vec<Option<T>>` fields.
- `Option<Option<T>>` writes `nullable` and `default` twice.
