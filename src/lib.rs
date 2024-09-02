#![allow(unreachable_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Time the duration of a function
#[proc_macro_attribute]
pub fn time_function(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Do nothing if release and not using the release feature
    #[cfg(all(not(debug_assertions), not(feature = "release")))]
    return input;

    // Parse the input token stream as a function
    let input = syn::parse_macro_input!(input as ItemFn);

    // Extract the function's signature and body
    let func_name = &input.sig.ident;
    let func_block = &input.block;
    let func_output = &input.sig.output;

    // Generate the wrapped function
    let output = quote! {
        fn #func_name() #func_output {
            let start = std::time::Instant::now();
            let result = (|| #func_block)();
            let duration: std::time::Duration = start.elapsed();
            println!("Function `{}` took {:?}", stringify!(#func_name), duration);
            result
        }
    };

    // Return the generated code as a token stream
    output.into()
}
