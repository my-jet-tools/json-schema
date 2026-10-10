use proc_macro2::TokenStream;
use types_reader::StructProperty;

/// `crate_path` - path to the `json-schema` crate as it is visible from the code which uses the macro
/// (e.g. `::json_schema` or `my_ai_agent::json_schema`).
pub fn generate(input: &syn::DeriveInput, crate_path: &TokenStream) -> Result<TokenStream, syn::Error> {
    //    let input_token_stream: proc_macro2::TokenStream = input.clone().into();

    let struct_name = &input.ident;

    let fields = StructProperty::read(input)?;

    let impl_deserialize_trait =
        super::generate_deserialize_trait(struct_name, fields.as_slice(), crate_path)?;

    let mut fields_to_render = Vec::new();

    let mut required_fields = Vec::new();

    for prop in fields {
        if !prop.ty.is_option() {
            required_fields.push(prop.name.to_string());
        }

        let property = super::generate_property(prop, crate_path)?;

        fields_to_render.push(property);
    }

    let required = if required_fields.len() > 0 {
        quote::quote! {  .write_iter("required", [#(#required_fields,)*].into_iter())}
    } else {
        quote::quote! {  .write("required", #crate_path::my_json::json_writer::EmptyJsonArray)}
    };

    let result = quote::quote! {

        impl #struct_name{

            pub async fn get_json_schema(output: bool)->#crate_path::my_json::json_writer::JsonObjectWriter{
                <Self as #crate_path::JsonTypeDescription>::get_description(false, None, output).await
            }
        }

        #impl_deserialize_trait

        #[#crate_path::async_trait::async_trait]
        impl #crate_path::JsonTypeDescription  for #struct_name{

        async fn get_description(_has_default: bool,
          with_enum: Option<Vec<#crate_path::rust_extensions::StrOrString<'static>>>,
          output: bool) -> #crate_path::my_json::json_writer::JsonObjectWriter {
        let props = #crate_path::my_json::json_writer::JsonObjectWriter::new()

        #(#fields_to_render)*;

        #crate_path::my_json::json_writer::JsonObjectWriter::new().write("type", "object")
        .write("properties", props )
        #required
        //.write("$schema", "http://json-schema.org/draft-07/schema#")
        //.write("title", #struct_name_as_string)
       //.write("additionalProperties", false)

       }

      }

    };
    Ok(result)
}
