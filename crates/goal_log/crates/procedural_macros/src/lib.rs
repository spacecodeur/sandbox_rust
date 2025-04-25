use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn description(message: TokenStream, function: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(function as ItemFn);

    let fn_sig = &sig;
    let fn_body = &block;
    let fn_attrs = &attrs;
    let fn_vis = &vis;

    let log_message = parse_macro_input!(message as LitStr).value();

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            goal_log::log_detailed(#log_message);

            #fn_body
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn tip(message: TokenStream, function: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(function as ItemFn);

    let fn_sig = &sig;
    let fn_body = &block;
    let fn_attrs = &attrs;
    let fn_vis = &vis;

    let log_message = parse_macro_input!(message as LitStr).value();

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            #fn_body
            goal_log::log_tip(#log_message);
        }
    };

    TokenStream::from(expanded)
}
