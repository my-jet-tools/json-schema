use proc_macro::TokenStream;

extern crate proc_macro;

#[proc_macro_derive(ApplyJsonSchema, attributes(property))]
pub fn apply_json_schema(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    match json_schema_macros_core::generate(&ast, &quote::quote!(::json_schema)) {
        Ok(result) => result.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
