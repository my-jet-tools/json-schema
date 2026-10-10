use proc_macro2::Ident;
use proc_macro2::TokenStream;
use types_reader::PropertyType;
use types_reader::StructProperty;

pub fn generate_deserialize_trait(
    struct_name: &Ident,
    fields: &[StructProperty],
    crate_path: &TokenStream,
) -> Result<TokenStream, syn::Error> {
    let mut init_null_props = vec![];

    let mut match_cases = vec![];

    let mut create_props = vec![];

    let mut null_verifications = vec![];

    for field in fields {
        let prop_name = field.get_field_name_ident();
        let prop_type = field.get_syn_type();
        let prop_name_as_str = prop_name.to_string();

        // Accumulators and the generated locals below are `__`-prefixed so a field
        // named e.g. `key` or `value` cannot collide with them.
        let field_var = quote::format_ident!("__field_{}", prop_name);

        init_null_props.push(quote::quote! {
            let mut #field_var = None;
        });

        create_props.push(quote::quote! {
            #prop_name: #field_var,
        });

        match &field.ty {
            PropertyType::OptionOf(tp) => {
                if let PropertyType::VecOf(items) = tp.as_ref() {
                    let tp = items.get_token_stream();
                    match_cases.push(quote::quote! {
                        #prop_name_as_str =>{
                             if let Some(__value) = __value.as_raw_str() {

                              if !__value.eq_ignore_ascii_case("null") {
                                 let __value: Vec<#tp> = #crate_path::deserializer::deserialize_array(__value)?;
                                 #field_var = Some(__value);
                              }
                        }
                        }
                    });
                } else {
                    let tp = tp.get_token_stream();
                    match_cases.push(quote::quote! {
                        #prop_name_as_str =>{
                            if let Some(__value) = __value.as_raw_str() {
                                if !__value.eq_ignore_ascii_case("null") {
                                  let __value = <#tp as #crate_path::deserializer::impl_from_str::DeserializeToolCallParam>::from_str(__value)?;
                                  #field_var = Some(__value);
                                }
                            }
                        }
                    });
                }
            }

            PropertyType::VecOf(items) => {
                let tp = items.get_token_stream();
                match_cases.push(quote::quote! {
                    #prop_name_as_str =>{
                         if __value.is_null() {
                             return Err(format!("Value of `{}` cannot be null", #prop_name_as_str));
                         }

                         if let Some(__value) = __value.as_raw_str() {
                             let __value: Vec<#tp> =
                            #crate_path::deserializer::deserialize_array(__value)?;
                        #field_var = Some(__value);
                    }
                    }
                });

                null_verifications.push(quote::quote! {
                    let Some(#field_var) = #field_var else {
                      return Err(format!("Json field `{}` is missing", #prop_name_as_str));
                    };
                });
            }

            _ => {
                match_cases.push(quote::quote! {
                    #prop_name_as_str =>{
                           // `as_raw_str` gives the text `null` for a null value, so it is checked separately
                           if __value.is_null() {
                                return Err(format!("Value of `{}` cannot be null", #prop_name_as_str));
                           }

                           let Some(__value) = __value.as_raw_str() else {
                                return Err(format!("Value of `{}` cannot be null", #prop_name_as_str));
                            };

                            let __value = <#prop_type as #crate_path::deserializer::impl_from_str::DeserializeToolCallParam>::from_str(__value)?;

                            #field_var = Some(__value);
                    },
                });

                null_verifications.push(quote::quote! {
                    let Some(#field_var) = #field_var else {
                      return Err(format!("Json field `{}` is missing", #prop_name_as_str));
                    };
                });
            }
        }
    }

    let result = quote::quote! {

        impl #crate_path::deserializer::impl_from_str::DeserializeToolCallParam for #struct_name {

            fn from_str(__src: &str) -> Result<Self, String> where Self: Sized,
            {
                #(#init_null_props)*

                let __json_iterator = #crate_path::my_json::json_reader::JsonFirstLineIterator::new(__src.as_bytes());

                while let Some(__next_item) = __json_iterator.get_next() {
                    let (__key, __value) = __next_item.map_err(|err| format!("{:?}", err))?;
                    let __key = __key.as_str().map_err(|err| format!("{:?}", err))?;

                     match __key.as_str() {
                        #(#match_cases)*
                        _ => {}
                        }

                }

                #(#null_verifications)*


                let __result = Self {#(#create_props)* };

                Ok(__result)
            }
        }
    };

    Ok(result)
}
