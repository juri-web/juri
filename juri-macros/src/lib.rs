extern crate proc_macro;
use generate::generate_struct;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs, ItemFn};
mod generate;
mod utils;

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let path = args[0].to_token_stream();
    let mut string = path.to_string();
    string = string[1..string.len() - 1].to_string();

    match syn::parse::<ItemFn>(item) {
        Ok(item_fn) => {
            let vis = item_fn.vis.clone();
            let ident = item_fn.sig.ident.clone();
            let def_struct = generate_struct(item_fn);
            let expanded = quote! {
                #vis fn #ident() -> juri::Route {
                    #def_struct

                    juri::Route {
                        method: juri::HTTPMethod::POST,
                        path: #string.to_string(),
                        handler: std::rc::Rc::new(#ident)
                    }
                }
            };
            expanded.into()
        }
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let path = args[0].to_token_stream();
    let mut string = path.to_string();
    string = string[1..string.len() - 1].to_string();

    match syn::parse::<ItemFn>(item) {
        Ok(item_fn) => {
            let vis = item_fn.vis.clone();
            let ident = item_fn.sig.ident.clone();
            let def_struct = generate_struct(item_fn);
            let expanded = quote! {
                #vis fn #ident() -> juri::Route {
                    #def_struct

                    juri::Route {
                        method: juri::HTTPMethod::POST,
                        path: #string.to_string(),
                        handler: std::rc::Rc::new(#ident)
                    }
                }
            };
            expanded.into()
        }
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match syn::parse::<ItemFn>(item) {
        Ok(item_fn) => generate_struct(item_fn).into(),
        Err(err) => err.into_compile_error().into(),
    }
}
