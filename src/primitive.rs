//! Primitive parsers for common proc-macro parsing operations

use chumsky::prelude::*;
use chumsky::error::Error;
use proc_macro2::{Punct, Spacing};

use crate::{RustToken, RustSpan};

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
                    if let RustToken::Punct(punct) = tok {
                        if punct.as_char() == puncts[idx].as_char() && punct.spacing() == puncts[idx].spacing() {
                            Ok(punct)
                        } else {
                            Err(E::expected_input_found(span.clone(), [], Some(RustToken::Punct(punct))))
                        }
                    } else {
                        Err(E::expected_input_found(span.clone(), [], Some(tok)))
                    }
                })
                .collect::<Result<_, _>>()
        })
        .chain(filter_map(move |span, tok| {
            if let RustToken::Punct(punct) = tok {
                if punct.as_char() == last.as_char() {
                    Ok(punct)
                } else {
                    Err(E::expected_input_found(span, [], Some(RustToken::Punct(punct))))
                }
            } else {
                Err(E::expected_input_found(span, [], Some(tok)))
            }
        }))
}
