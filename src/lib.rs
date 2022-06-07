//! Use chumsky in your proc-macros with chumsky-proc!
//!
//! Simply write your parsers which take streams of `RustToken`, then call
//! `stream_from_tokens` with a `TokenStream` to generate a stream that can be passed
//! to your parsers. Easy as pie!

#![warn(
    missing_docs,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    missing_abi,
    noop_method_call,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    unused_import_braces,
    unused_lifetimes,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::ptr_as_ptr,
    clippy::cloned_instead_of_copied,
    clippy::unreadable_literal
)]

use proc_macro2::{TokenTree, TokenStream};

mod span;
mod token;
mod regular;
pub mod primitive;
// TODO: zero-copy, once it's released

pub use span::RustSpan;
pub use token::RustToken;
pub use regular::*;

fn into_vec(stream: TokenStream) -> Vec<(RustToken, RustSpan)> {
    stream.into_iter()
        .flat_map(|tree| match tree {
            TokenTree::Group(group) => {
                let inner = into_vec(group.stream());
                let mut out = Vec::with_capacity(inner.len() + 2);
                out.push((RustToken::StartDelim(group.delimiter()), group.span_open().into()));
                out.extend(inner);
                out.push((RustToken::EndDelim(group.delimiter()), group.span_close().into()));
                out
            }
            TokenTree::Ident(ident) => {
                let span = ident.span().into();
                vec![(RustToken::Ident(ident), span)]
            },
            TokenTree::Punct(punct) => {
                let span = punct.span().into();
                vec![(RustToken::Punct(punct), span)]
            },
            TokenTree::Literal(lit) => {
                let span = lit.span().into();
                vec![(RustToken::Literal(lit), span)]
            },
        })
        .collect()
}
