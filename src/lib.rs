#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::cargo)]
#![forbid(unsafe_code)]
#![allow(unreachable_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Time the duration of a function, either to stdout or via `tracing`.
#[proc_macro_attribute]
pub fn time_function(
    #[allow(unused_variables)] args: TokenStream,
    input: TokenStream,
) -> TokenStream {
    // Do nothing if release and not using the release feature
    // Do nothing if not testing when using the testing feature
    #[cfg(any(
        all(not(debug_assertions), not(feature = "release")),
        all(not(test), feature = "testing")
    ))]
    return input;

    // Parse the input token stream as a function
    let input = syn::parse_macro_input!(input as ItemFn);

    // Extract the function's signature and body
    let func_name = &input.sig.ident;
    let func_block = &input.block;
    let func_output = &input.sig.output;
    let func_input = &input.sig.inputs;
    let func_vis = &input.vis;

    let func_label = if args.is_empty() {
        format!("{func_name}()")
    } else {
        args.to_string()
    };

    // Generate the wrapped function
    let output = if input.sig.asyncness.is_some() {
        quote! {
            async #func_vis fn #func_name(#func_input) #func_output {
                let start = std::time::Instant::now();
                let result = (|| async #func_block)().await;
                let duration: std::time::Duration = start.elapsed();
                #[cfg(not(feature = "tracing"))]
                println!("`{}` took {:?}", #func_label, duration);
                #[cfg(feature = "tracing")]
                tracing::trace!("`{}` took {:?}", #func_label, duration);
                result
            }
        }
    } else {
        quote! {
            #func_vis fn #func_name(#func_input) #func_output {
                let start = std::time::Instant::now();
                let result = (|| #func_block)();
                let duration: std::time::Duration = start.elapsed();
                #[cfg(not(feature = "tracing"))]
                println!("`{}` took {:?}", #func_label, duration);
                #[cfg(feature = "tracing")]
                tracing::trace!("`{}` took {:?}", #func_label, duration);
                result
            }
        }
    };

    // Return the generated code as a token stream
    output.into()
}

/// Time the duration of code snippet, either to stdout or via `tracing`.
#[proc_macro]
pub fn time_snippet(input: TokenStream) -> TokenStream {
    // Do nothing if release and not using the release feature
    #[cfg(all(not(debug_assertions), not(feature = "release")))]
    return input;

    let block: proc_macro2::token_stream::TokenStream = input.into();

    let output = quote! {
        {
            let begin = line!();
            let start = std::time::Instant::now();
            let result =
                #block;
            let duration: std::time::Duration = start.elapsed();
            #[cfg(not(feature = "tracing"))]
            println!("{}:{} took {:?}.", file!(), begin, duration);
            #[cfg(feature = "tracing")]
            tracing::trace!("{}:{} took {:?}.", file!(), begin, duration);
            result
        }
    };

    output.into()
}
