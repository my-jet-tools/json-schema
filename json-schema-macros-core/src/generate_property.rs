use std::str::FromStr;

use proc_macro2::TokenStream;
use types_reader::{AnyValueAsStr, StructProperty};

pub fn generate_property(
    prop: StructProperty,
    crate_path: &TokenStream,
) -> Result<TokenStream, syn::Error> {
    let prop_name = prop.name.as_str();
    let prop_type = prop.get_syn_type();
    let attr = prop.attrs.get_attr("property")?;

    let enum_param = attr.try_get_named_param("enum");

    let (write_default, has_default) = if let Some(default_value) =
        attr.try_get_named_param("default")
    {
        let default_value = default_value.unwrap_as_value()?.as_string()?;
        let default_value = default_value.as_str();

        (
            quote::quote! {
               .write("default", <#prop_type as #crate_path::JsonTypeDescription>::schema_value(#default_value.into()))
            },
            true,
        )
    } else {
        (quote::quote! {}, false)
    };

    let enum_to_render = match enum_param {
        Some(enum_value) => {
            if let Ok(array) = enum_value.unwrap_as_vec() {
                let mut array_as_tokens = Vec::new();

                for itm in array {
                    let as_str = itm.unwrap_as_value()?.as_str()?;
                    array_as_tokens.push(quote::quote! {#as_str.into(), });
                }

                quote::quote! {Some(vec![#(#array_as_tokens)*])}
            } else {
                let as_str = enum_value.unwrap_as_value()?.as_str()?;

                let enum_fn_name = TokenStream::from_str(as_str).unwrap();
                quote::quote! {
                    #enum_fn_name().await
                }
            }
        }
        None => quote::quote! {None},
    };

    let description = attr.get_value_from_single_or_named("description")?;
    let description = description.as_string()?;
    let description = description.as_str();

    let result = quote::quote! {
        .write(#prop_name, <#prop_type as #crate_path::JsonTypeDescription>::get_description(#has_default, #enum_to_render, output).await.write("description", #description) #write_default)
    };

    Ok(result)
}
