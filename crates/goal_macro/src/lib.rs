use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};

// #[proc_macro_attribute]
// pub fn goal(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let message = parse_macro_input!(attr as LitStr);

//     let input_fn = parse_macro_input!(item as ItemFn);

//     let fn_name = &input_fn.sig.ident;

//     let generated = quote! {
//         #input_fn

//         #[allow(unused_variables)]
//         fn #fn_name() {
//             println!("GOAL - {} : {}", stringify!(#fn_name), #message);
//             #input_fn
//         }
//     };

//     TokenStream::from(generated)
// }

#[macro_export]
macro_rules! goal {
    ($message:expr, $func:path) => {{
        println!("GOAL - {} : {}", stringify!($func), $message);
        $func();
    }};
}

#[proc_macro_attribute]
pub fn description(message: TokenStream, function: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(function as ItemFn);

    let fn_sig = &sig;
    let fn_name = &sig.ident;
    let fn_body = &block;
    let fn_attrs = &attrs;
    let fn_vis = &vis;

    let log_message = parse_macro_input!(message as LitStr).value();

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            use colored::*;

            println!(
                "[ {} : {} ] => {}",
                "GOAL".bold(),
                format!("{}", stringify!(#fn_name)).bold().green(),
                #log_message.bold().blue()
            );

            #fn_body
        }
    };

    TokenStream::from(expanded)
}
