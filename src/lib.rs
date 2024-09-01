use proc_macro::TokenStream;
use quote::ToTokens;
use quote::quote;

/// Time the duration of a function
#[proc_macro_attribute]
pub fn time_function(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).unwrap();

    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected function!")
    };

    fn_item.block.stmts.insert(0,syn::parse(quote!(let start = std::time::Instant::now();).into()).expect("Failed to insert start time."));

    let end = fn_item.block.stmts.len();
    let fn_name = fn_item.sig.ident.to_string();

    fn_item.block.stmts.insert(end,syn::parse(quote!(let duration: std::time::Duration = start.elapsed();).into()).expect("Failed to insert end time."));

    let end = fn_item.block.stmts.len();
    fn_item.block.stmts.insert(end,syn::parse(quote!(println!("Time elapsed in {}(): {:?}", #fn_name, duration);).into()).expect("Failed to end print statement."));

    item.into_token_stream().into()
}
