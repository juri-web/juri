extern crate proc_macro;
use generate::{generate_struct, generate_ws_struct};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Meta, NestedMeta};
mod generate;
mod utils;

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);

    let mut internal = false;
    let mut ws = false;
    for arg in &args {
        if matches!(arg, NestedMeta::Meta(Meta::Path(p)) if p.is_ident("internal")) {
            internal = true;
        } else if matches!(arg, NestedMeta::Meta(Meta::Path(p)) if p.is_ident("ws")) {
            ws = true;
        }
    }

    let path = args[0].to_token_stream();
    let mut string = path.to_string();
    string = string[1..string.len() - 1].to_string();
    let crate_name = utils::get_crate_name(internal);

    match syn::parse::<ItemFn>(item) {
        Ok(item_fn) => {
            let vis = item_fn.vis.clone();
            let ident = item_fn.sig.ident.clone();
            let def_struct = if ws {
                generate_ws_struct(internal, item_fn)
            } else {
                generate_struct(internal, item_fn)
            };
            let expanded = if ws {
                quote! {
                    #vis fn #ident() -> #crate_name::RouteOrWSRoute {
                        #def_struct

                        #crate_name::RouteOrWSRoute::WS(#crate_name::WSRoute {
                            path: #string.to_string(),
                            handler: std::sync::Arc::new(#ident)
                        })
                    }
                }
            } else {
                quote! {
                    #vis fn #ident() -> #crate_name::RouteOrWSRoute {
                        #def_struct

                        #crate_name::RouteOrWSRoute::Common(#crate_name::Route {
                            method: #crate_name::HTTPMethod::GET,
                            path: #string.to_string(),
                            handler: std::sync::Arc::new(#ident)
                        })
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

    let mut internal = false;
    for arg in &args {
        if matches!(arg, NestedMeta::Meta(Meta::Path(p)) if p.is_ident("internal")) {
            internal = true;
        }
    }

    let path = args[0].to_token_stream();
    let mut string = path.to_string();
    string = string[1..string.len() - 1].to_string();
    let crate_name = utils::get_crate_name(internal);

    match syn::parse::<ItemFn>(item) {
        Ok(item_fn) => {
            let vis = item_fn.vis.clone();
            let ident = item_fn.sig.ident.clone();
            let def_struct = generate_struct(internal, item_fn);
            let expanded = quote! {
                #vis fn #ident() -> #crate_name::RouteOrWSRoute {
                    #def_struct

                    #crate_name::RouteOrWSRoute::Common(#crate_name::Route {
                        method: #crate_name::HTTPMethod::POST,
                        path: #string.to_string(),
                        handler: std::sync::Arc::new(#ident)
                    })
                }
            };
            expanded.into()
        }
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);

    let mut internal = false;
    for arg in &args {
        if matches!(arg, NestedMeta::Meta(Meta::Path(p)) if p.is_ident("internal")) {
            internal = true;
        }
    }

    match syn::parse::<ItemFn>(item) {
        Ok(item_fn) => generate_struct(internal, item_fn).into(),
        Err(err) => err.into_compile_error().into(),
    }
}
