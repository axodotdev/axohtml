#![recursion_limit = "128"]
#![cfg_attr(can_show_location_of_runtime_parse_error, feature(proc_macro_span))]

#[allow(unused_imports)]
use rust_format::Formatter;

extern crate proc_macro;

mod config;
mod declare;
mod error;
mod html;
mod ident;
mod lexer;
mod map;
mod parser;
mod span;

/// Construct a DOM tree.
///
/// See the crate documentation for [`axohtml`][axohtml].
///
/// [axohtml]: https://docs.rs/axohtml/
#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    html_impl(input.into()).into()
}
fn html_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let stream = lexer::unroll_stream(input, false);
    let result = html::expand_html(&stream);
    match result {
        Err(err) => error::parse_error(&stream, &err),
        Ok((node, ty)) => match node.into_token_stream(&ty) {
            Err(err) => err,
            Ok(success) => success,
        },
    }
}

/// Construct a Dodrio node.
///
/// See the crate documentation for [`axohtml`][axohtml].
///
/// [axohtml]: https://docs.rs/axohtml/
#[cfg(feature = "dodrio")]
#[proc_macro]
pub fn dodrio(input: TokenStream) -> TokenStream {
    let stream = lexer::unroll_stream(input.into(), false);
    let result = html::expand_dodrio(&stream);
    TokenStream::from(match result {
        Err(err) => error::parse_error(&stream, &err),
        Ok((bump, node)) => match node.into_dodrio_token_stream(&bump, false) {
            Err(err) => err,
            // Ok(success) => {println!("{}", success); panic!()},
            Ok(success) => success,
        },
    })
}

/// This macro is used by `axohtml` internally to generate types and
/// implementations for HTML elements.
#[proc_macro]
pub fn declare_elements(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    declare_elements_impl(input.into()).into()
}

fn declare_elements_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let stream = lexer::keywordise(lexer::unroll_stream(input, true));
    let result = declare::expand_declare(&stream);
    match result {
        Err(err) => error::parse_error(&stream, &err),
        Ok(decls) => {
            let mut out = proc_macro2::TokenStream::new();
            for decl in decls {
                out.extend(decl.into_token_stream());
            }
            out
        }
    }
}

#[test]
fn test_declare_anchor_element() {
    use quote::quote;

    let input = quote!(
        a {
            download: String,
            href: Uri,
            hreflang: LanguageTag,
            ping: SpacedList<Uri>,
            rel: SpacedList<LinkType>,
            target: Target,
            type: Mime,
        } in [FlowContent, PhrasingContent, InteractiveContent] with FlowContent;
    );

    let output = declare_elements_impl(input);

    pretty_assertions::assert_eq!(
        rust_format::PrettyPlease::default().format_str(output.to_string()).unwrap(),
        include_str!("tests/declare-anchor-element.rs")
    )
}

#[test]
fn test_html_anchor_element() {
    use quote::quote;

    let input = quote!(
        <a href="https://www.w3schools.com">
            "Visit W3Schools.com!"
        </a>
    );

    let output = html_impl(input);
    let output = format!("fn html() {}", output.to_string());

    pretty_assertions::assert_eq!(
        rust_format::PrettyPlease::default().format_str(output.to_string()).unwrap(),
        include_str!("tests/generate-anchor-element.rs")
    )
}

#[test]
fn test_html_button_event() {
    use quote::quote;

    let input = quote!(
        <button onclick="alert(1)">
            "Click me"
        </button>
        : String
    );

    let output = html_impl(input);
    let output = format!("fn html() {}", output.to_string());

    pretty_assertions::assert_eq!(
        rust_format::PrettyPlease::default().format_str(output.to_string()).unwrap(),
        include_str!("tests/generate-button-event.rs")
    )
}