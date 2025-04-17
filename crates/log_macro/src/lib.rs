use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn log(_attribut: TokenStream, element: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(element as ItemFn);

    let fn_sig = &sig;
    let fn_name = &sig.ident;
    let fn_body = &block;
    let fn_attrs = &attrs;
    let fn_vis = &vis;

    let log_message = parse_macro_input!(_attribut as LitStr).value();

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            use colored::*;

            println!(
                "{} {}",
                format!("→ Function `{}`:", stringify!(#fn_name)).bold().blue(),
                #log_message.bold().yellow()
            );

            #fn_body
        }
    };

    TokenStream::from(expanded)
}
