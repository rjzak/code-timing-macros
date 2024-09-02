#![allow(unreachable_code)]

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{Expr, ExprBlock, Stmt};

/// Time the duration of a function
#[proc_macro_attribute]
pub fn time_function(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Do nothing if release and not using the release feature
    #[cfg(all(not(debug_assertions), not(feature = "release")))]
    return input;

    let mut item: syn::Item = syn::parse(input).unwrap();

    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected function!"),
    };

    // Put the original code in it's own block to prevent variable scope issues.
    // TODO: Return the value returned by the function.
    let original_function_block = Stmt::Expr(Expr::Block(ExprBlock {
        attrs: fn_item.attrs.clone(),
        label: None,
        block: fn_item.block.as_ref().clone(),
    }));
    let fn_name = fn_item.sig.ident.to_string();

    fn_item.block.stmts.clear();
    fn_item.block.stmts.push(
        syn::parse(quote!(let start = std::time::Instant::now();).into())
            .expect("Failed to insert start time."),
    );
    fn_item.block.stmts.push(original_function_block);
    fn_item.block.stmts.push(
        syn::parse(quote!(let duration: std::time::Duration = start.elapsed();).into())
            .expect("Failed to insert end time."),
    );
    fn_item.block.stmts.push(
        syn::parse(quote!(println!("Time elapsed in {}(): {:?}", #fn_name, duration);).into())
            .expect("Failed to end print statement."),
    );

    item.into_token_stream().into()
}
