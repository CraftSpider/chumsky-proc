//! Primitive parsers for common proc-macro parsing operations

use chumsky::error::Error;
use chumsky::prelude::*;
use proc_macro2::{Ident, Punct, Spacing, Span};

use crate::utils::punct_eq;
use crate::{RustSpan, RustToken};

/// Accepts only an exact identifier, output `()` on success
///
/// # Examples
///
/// ```
/// # use chumsky_proc::primitive::keyword;
/// # use chumsky_proc::{stream_from_tokens, RustToken, RustSpan};
/// # use chumsky::Parser;
/// # use chumsky::primitive::filter_map;
/// # use chumsky::error::Cheap;
/// # use quote::quote;
/// let parser = keyword::<Cheap<_, RustSpan>>("struct")
///     .ignore_then(filter_map(RustToken::filter_ident));
///
/// parser.parse(stream_from_tokens(quote!(struct Foo)))
///     .unwrap();
///
/// parser.parse(stream_from_tokens(quote!(enum Foo)))
///     .unwrap_err();
/// ```
pub fn keyword<'a, E: 'a + Error<RustToken, Span = RustSpan>>(
    keyword: &'a str,
) -> impl Parser<RustToken, (), Error = E> + Clone + 'a {
    filter_map(move |span, tok: RustToken| {
        tok.into_ident()
            .and_then(|ident| {
                if ident == keyword {
                    Ok(())
                } else {
                    Err(RustToken::Ident(ident))
                }
            })
            .map_err(|tok| {
                E::expected_input_found(
                    span,
                    [Some(RustToken::Ident(Ident::new(
                        keyword,
                        Span::mixed_site(),
                    )))],
                    Some(tok),
                )
            })
    })
}

/// Accepts a single punctuation token, joined or not
///
/// # Examples
///
/// ```
/// # use chumsky_proc::primitive::punct;
/// # use chumsky_proc::{stream_from_tokens, RustToken, RustSpan};
/// # use chumsky::Parser;
/// # use chumsky::primitive::filter_map;
/// # use chumsky::error::Cheap;
/// # use quote::quote;
/// let parser = filter_map::<_, _, _, Cheap<_, RustSpan>>(RustToken::filter_ident)
///     .then_ignore(punct('+'))
///     .then(filter_map(RustToken::filter_ident));
///
/// parser.parse(stream_from_tokens(quote!(a + b)))
///     .unwrap();
///
/// parser.parse(stream_from_tokens(quote!(a - b)))
///     .unwrap_err();
/// ```
pub fn punct<E: Error<RustToken, Span = RustSpan>>(
    c: char,
) -> impl Parser<RustToken, (), Error = E> + Clone {
    filter_map(move |span, tok: RustToken| {
        tok.into_punct()
            .and_then(|punct| {
                if punct.as_char() == c {
                    Ok(())
                } else {
                    Err(RustToken::Punct(punct))
                }
            })
            .map_err(|tok| {
                E::expected_input_found(
                    span,
                    [Some(RustToken::Punct(Punct::new(c, Spacing::Alone)))],
                    Some(tok),
                )
            })
    })
}

/// Generate a parser for a series of joined punct tokens, with the ending allowing any spacing.
/// Given `"+="`, this will match `+=` and `+=+`, but not `+ =`.
///
/// # Panics
///
/// If the provided punctuation string is empty
///
/// # Examples
///
/// ```
/// # use chumsky_proc::primitive::joined_punct;
/// # use chumsky_proc::{stream_from_tokens, RustSpan};
/// # use chumsky::Parser;
/// # use chumsky::error::Cheap;
/// # use quote::quote;
/// let parser = joined_punct::<Cheap<_, RustSpan>>("+=");
///
/// parser.parse(stream_from_tokens(quote!(+=))).unwrap();
///
/// parser.parse(stream_from_tokens(quote!(+=+))).unwrap();
///
/// parser.parse(stream_from_tokens(quote!(+ =))).unwrap_err();
/// ```
///
pub fn joined_punct<E: Error<RustToken, Span = RustSpan>>(
    punct: &str,
) -> impl Parser<RustToken, Vec<Punct>, Error = E> + Clone {
    use chumsky::prelude::*;

    if punct.is_empty() {
        panic!("Invalid punctuation for Rust proc-macro");
    }

    let mut puncts = punct
        .chars()
        .map(|c| Punct::new(c, Spacing::Joint))
        .collect::<Vec<_>>();

    let last = puncts.pop().unwrap();

    any()
        .repeated()
        .exactly(puncts.len())
        .try_map::<Vec<Punct>, _>(move |toks: Vec<RustToken>, span: RustSpan| {
            toks.into_iter()
                .enumerate()
                .map(|(idx, tok)| {
                    tok.into_punct()
                        .and_then(|punct| {
                            if punct_eq(&punct, &puncts[idx]) {
                                Ok(punct)
                            } else {
                                Err(RustToken::Punct(punct))
                            }
                        })
                        .map_err(|tok| E::expected_input_found(span.clone(), [], Some(tok)))
                })
                .collect::<Result<_, _>>()
        })
        .chain(filter_map(move |span, tok: RustToken| {
            tok.into_punct()
                .and_then(|punct| {
                    if punct.as_char() == last.as_char() {
                        Ok(punct)
                    } else {
                        Err(RustToken::Punct(punct))
                    }
                })
                .map_err(|tok| E::expected_input_found(span, [], Some(tok)))
        }))
}
