use crate::utils;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn generate_struct(internal: bool, item_fn: ItemFn) -> TokenStream {
    let crate_name = utils::get_crate_name(internal);
    let vis = item_fn.vis.clone();
    let ident = item_fn.sig.ident.clone();
    let call_await = if item_fn.sig.asyncness.is_some() {
        Some(quote::quote!(.await))
    } else {
        None
    };

    let def_struct = quote! {
        struct #ident;
    };
    let expanded = quote! {
        #[allow(non_camel_case_types)]
        #vis #def_struct

        #[#crate_name::async_trait]
        impl #crate_name::HTTPHandler for #ident {
            async fn call(&self, request: &#crate_name::Request) -> #crate_name::Result<#crate_name::Response> {
                #item_fn
                let res = #ident(&request)#call_await;
                res
            }
        }
    };
    expanded.into()
}
