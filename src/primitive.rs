//! Primitive parsers for common proc-macro parsing operations

use chumsky::prelude::*;
use chumsky::error::Error;
use proc_macro2::{Punct, Spacing};

use crate::{RustToken, RustSpan};
use crate::utils::punct_eq;

/// Generate a parser for a series of joined punct tokens, with the ending allowing any spacing.
/// Given `"+="`, this will match `+=` and `+=+`, but not `+ =`.
///
/// # Examples
///
/// ```
/// # use chumsky_proc::primitive::joined_punct;
/// # use chumsky_proc::{stream_from_tokens, RustToken, RustSpan};
/// # use proc_macro2::{Punct, Spacing};
/// # use chumsky::Parser;
/// # use quote::quote;
/// let parser = joined_punct::<chumsky::error::Cheap<_, RustSpan>>("+=");
///
/// parser.parse(stream_from_tokens(quote!(+=))).unwrap();
///
/// parser.parse(stream_from_tokens(quote!(+=+))).unwrap();
///
/// parser.parse(stream_from_tokens(quote!(+ =))).unwrap_err();
/// ```
///
pub fn joined_punct<E: Error<RustToken, Span = RustSpan>>(punct: &str) -> impl Parser<RustToken, Vec<Punct>, Error = E> {
    use chumsky::prelude::*;

    if punct.is_empty() {
        panic!("Invalid punctuation for Rust proc-macro");
    }

    let mut puncts = punct.chars()
        .map(|c| {
            Punct::new(c, Spacing::Joint)
        })
        .collect::<Vec<_>>();

    let last = puncts.pop().unwrap();

    any()
        .repeated()
        .exactly(puncts.len())
        .try_map::<Vec<Punct>, _>(move |toks: Vec<RustToken>, span: RustSpan| {
            toks.into_iter().enumerate()
                .map(|(idx, tok)| {
                    tok.into_punct()
                        .and_then(|punct| if punct_eq(&punct, &puncts[idx]) {
                            Ok(punct)
                        } else {
                            Err(RustToken::Punct(punct))
                        })
                        .map_err(|tok| E::expected_input_found(span.clone(), [], Some(tok)))
                })
                .collect::<Result<_, _>>()
        })
        .chain(filter_map(move |span, tok: RustToken| {
            tok.into_punct()
                .and_then(|punct| if punct.as_char() == last.as_char() {
                    Ok(punct)
                } else {
                    Err(RustToken::Punct(punct))
                })
                .map_err(|tok| E::expected_input_found(span, [], Some(tok)))
        }))
}
