#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(unreachable_code, unused_variables)]

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quote::{quote, quote_spanned};
use syn::ItemFn;

/// Time the duration of a function, either to stdout or via `tracing`.
/// If using the `tracing` feature, ensure the tracing crate is in
/// your `Cargo.toml`!
///
/// ```
/// use code_timing_macros::time_function;
///
/// #[time_function]
/// fn my_cool_function() -> f32 {
///     3.14 * 10.0 / 2.3
/// }
///
/// #[time_function("just testing something")]
/// pub fn my_other_cool_function() -> f32 {
///     3.14 * 10.0 / 2.3
/// }
///
/// fn main() {
///     let _ = my_cool_function();
/// }
/// ```
#[proc_macro_attribute]
pub fn time_function(
    #[allow(unused_variables)] args: TokenStream,
    input: TokenStream,
) -> TokenStream {
    // Do nothing if release and not using the release feature
    #[cfg(all(not(debug_assertions), not(feature = "release")))]
    return input;

    // Parse the input token stream as a function
    let input = syn::parse_macro_input!(input as ItemFn);

    // Extract the function's signature and body
    let func_name = &input.sig.ident;
    let func_block = &input.block;
    let func_output = &input.sig.output;
    let func_input = &input.sig.inputs;
    let func_vis = &input.vis;

    let args: Vec<TokenTree> = proc_macro2::TokenStream::from(args).into_iter().collect();

    let func_label = match args.as_slice() {
        // No argument, so name the function after itself
        [] => {
            let label = format!("{func_name}()");
            quote! { #label }
        }
        // A string literal is the label, with rustc unquoting it for us
        [TokenTree::Literal(label)] if is_string_literal(label) => quote! { #label },
        // Keep the function itself, so only the label is reported as wrong
        _ => {
            let args: proc_macro2::TokenStream = args.iter().cloned().collect();
            let error = syn::Error::new_spanned(
                args,
                "the label must be a string literal, such as `#[time_function(\"SomeObject::new()\")]`",
            )
            .to_compile_error();
            return quote! { #error #input }.into();
        }
    };

    let log_stmt = if cfg!(feature = "tracing") {
        quote! { ::tracing::trace!("`{}` took {:?}", #func_label, duration); }
    } else {
        quote! { println!("`{}` took {:?}", #func_label, duration); }
    };

    // Generate the wrapped function
    let output = if input.sig.asyncness.is_some() {
        quote! {
            #func_vis async fn #func_name(#func_input) #func_output {
                let start = ::std::time::Instant::now();
                let result = (|| async #func_block)().await;
                let duration: ::std::time::Duration = start.elapsed();
                #log_stmt
                result
            }
        }
    } else {
        quote! {
            #func_vis fn #func_name(#func_input) #func_output {
                let start = ::std::time::Instant::now();
                let result = (|| #func_block)();
                let duration: ::std::time::Duration = start.elapsed();
                #log_stmt
                result
            }
        }
    };

    // Return the generated code as a token stream
    output.into()
}

/// Time the duration of a code snippet, either to stdout or via `tracing`.
/// If using the `tracing` feature, ensure the tracing crate is in
/// your `Cargo.toml`!
///
/// ```
/// use code_timing_macros::time_snippet;
///
/// time_snippet!({
///     std::thread::sleep(std::time::Duration::from_secs_f32(1.0f32))
/// })
///
/// ```
#[proc_macro]
pub fn time_snippet(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = proc_macro2::TokenStream::from(input).into_iter().collect();

    // Check for a leading string literal, which would be the label.
    let (label, skip) = match (tokens.first(), tokens.get(1)) {
        (Some(TokenTree::Literal(label)), Some(TokenTree::Punct(comma)))
            if comma.as_char() == ',' && is_string_literal(label) =>
        {
            (Some(label.clone()), 2)
        }
        _ => (None, 0),
    };

    let body = &tokens[skip..];
    let block: proc_macro2::TokenStream = body.iter().cloned().collect();

    // Do nothing if release and not using the release feature
    #[cfg(all(not(debug_assertions), not(feature = "release")))]
    return block.into();

    // `begin` and `end` are only needed when falling back to the line numbers
    let line_statements = if label.is_some() {
        quote! {}
    } else {
        // `line!()` reports the line of its own span, so give each call the
        // span of the token it should name
        let begin = line_of(body.first().map(opening_span));
        let end = line_of(body.last().map(closing_span));
        quote! {
            let begin = #begin;
            let end = #end;
        }
    };

    let log_stmt = match (&label, cfg!(feature = "tracing")) {
        (Some(label), true) => quote! { ::tracing::trace!("`{}` took {:?}.", #label, duration); },
        (Some(label), false) => quote! { println!("`{}` took {:?}.", #label, duration); },
        (None, true) => {
            quote! { ::tracing::trace!("{}:{}-{} took {:?}.", file!(), begin, end, duration); }
        }
        (None, false) => {
            quote! { println!("{}:{}-{} took {:?}.", file!(), begin, end, duration); }
        }
    };

    let output = quote! {
        {
            #line_statements
            let start = ::std::time::Instant::now();
            let result =
                #block;
            let duration: ::std::time::Duration = start.elapsed();
            #log_stmt
            result
        }
    };

    output.into()
}

/// Is this literal token a string literal, plain or raw? Only those make
/// sense as a label, and `r` is the only prefix they can have.
fn is_string_literal(literal: &proc_macro2::Literal) -> bool {
    let text = literal.to_string();
    text.starts_with('"') || text.starts_with('r')
}

/// Where a snippet starts: the opening delimiter of a block, or the token
/// itself when it isn't one.
fn opening_span(token: &TokenTree) -> Span {
    match token {
        TokenTree::Group(group) => group.span_open(),
        token => token.span(),
    }
}

/// Where a snippet ends: the closing delimiter of a block, or the token
/// itself when it isn't one.
fn closing_span(token: &TokenTree) -> Span {
    match token {
        TokenTree::Group(group) => group.span_close(),
        token => token.span(),
    }
}

/// A call to `line!()` reporting the line of the given span, falling back to
/// the line of the macro invocation for an empty snippet.
fn line_of(span: Option<Span>) -> proc_macro2::TokenStream {
    span.map_or_else(
        || quote! { line!() },
        |span| quote_spanned! { span => line!() },
    )
}
