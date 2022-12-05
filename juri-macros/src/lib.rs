extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{parse_macro_input, AttributeArgs, Item, ItemFn, Result};
mod utils;

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let path = args[0].to_token_stream();
    let mut string = path.to_string();
    string = string[1..string.len() - 1].to_string();

    let input = parse_macro_input!(item as Item);
    let token_stream = input.to_token_stream();

    let mut token = token_stream.into_iter();
    let mut is_appear_fn = false;
    let mut is_appear_pub = false;
    let fn_name = loop {
        if let Some(data) = token.next() {
            if data.to_string() == "fn" {
                is_appear_fn = true;
            } else if is_appear_fn {
                break Some(data);
            } else if data.to_string() == "pub" {
                is_appear_pub = true;
            }
        } else {
            break None;
        }
    };
    let new_token_stream;
    if is_appear_pub {
        new_token_stream = quote!(
            pub fn #fn_name() -> juri::Route {
                #input
                (juri::HTTPMethod::GET, #string.to_string(), #fn_name)
            }
        )
        .into();
    } else {
        new_token_stream = quote!(
            fn #fn_name() -> juri::Route {
                #input
                (juri::HTTPMethod::GET, #string.to_string(), #fn_name)
            }
        )
        .into();
    }
    new_token_stream
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let path = args[0].to_token_stream();
    let mut string = path.to_string();
    string = string[1..string.len() - 1].to_string();

    let input = parse_macro_input!(item as ItemFn);
    let token_stream = input.to_token_stream();

    let mut token = token_stream.into_iter();
    let mut is_appear_fn = false;
    let mut is_appear_pub = false;
    let fn_name = loop {
        if let Some(data) = token.next() {
            if data.to_string() == "fn" {
                is_appear_fn = true;
            } else if is_appear_fn {
                break Some(data);
            } else if data.to_string() == "pub" {
                is_appear_pub = true;
            }
        } else {
            break None;
        }
    };
    let new_token_stream;
    if is_appear_pub {
        new_token_stream = quote!(
            pub fn #fn_name() -> juri::Route {
                #input
                juri::Route {
                    method: juri::HTTPMethod::POST,
                    path: #string.to_string(),
                    handle
                }
            }
        )
        .into();
    } else {
        new_token_stream = quote!(
            fn #fn_name() -> juri::Route {
                #input
                (juri::HTTPMethod::POST, #string.to_string(), #fn_name)
            }
        )
        .into();
    }
    new_token_stream
}


fn generate_struct(input: TokenStream) -> Result<TokenStream> {
    let crate_name = utils::get_crate_name(false);
    let item_fn = syn::parse::<ItemFn>(input)?;
    let vis = item_fn.vis;

    // let def_struct = quote! {

    // }
    let expanded = quote! {
    };
    Ok(expanded.into())
}