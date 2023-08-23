//! Primitive parsers for common proc-macro parsing operations

use chumsky::zero_copy::error::Error;
use chumsky::zero_copy::prelude::*;
use proc_macro2::{Ident, Punct, Spacing, Span};

use crate::utils::punct_eq;
use crate::{RustToken, RustTokens};

/// Accepts only an exact identifier, output `()` on success
///
/// # Examples
///
/// ```
/// # use chumsky_proc::prelude::*;
/// # use chumsky::prelude::*;
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
#[must_use]
pub fn keyword<'a, E, S>(
    keyword: &'a str,
) -> impl Parser<'a, RustTokens, (), E, S> + Clone + 'a
where
    E: 'a + Error<RustTokens>,
    S: 'a,
{
    any::<RustTokens, _, _>().try_map(move |tok, span| {
        tok.into_ident()
            .and_then(|ident| {
                if ident == keyword {
                    Ok(())
                } else {
                    Err(RustToken::Ident(ident))
                }
            })
            .map_err(|tok| {
                E::expected_found(
                    [Some(RustToken::Ident(Ident::new(
                        keyword,
                        Span::mixed_site(),
                    )))],
                    Some(tok),
                    span,
                )
            })
    })
}

/// Accepts a single punctuation token, joined or not
///
/// # Examples
///
/// ```
/// # use chumsky_proc::prelude::*;
/// # use chumsky::prelude::*;
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
#[must_use]
pub fn punct<'a, E, S>(
    c: char,
) -> impl Parser<'a, RustTokens, (), E, S> + Clone
where
    E: Error<RustTokens>,
    S: 'a,
{
    any::<RustTokens, _, _>().try_map(move |tok, span| {
        tok.into_punct()
            .and_then(|punct| {
                if punct.as_char() == c {
                    Ok(())
                } else {
                    Err(RustToken::Punct(punct))
                }
            })
            .map_err(|tok| {
                E::expected_found(
                    [Some(RustToken::Punct(Punct::new(c, Spacing::Alone)))],
                    Some(tok),
                    span,
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
/// # use chumsky_proc::prelude::*;
/// # use chumsky::prelude::*;
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
#[must_use]
pub fn joined_punct<'a, E, S>(
    punct: &str,
) -> impl Parser<'a, RustTokens, Vec<Punct>, E, S> + Clone
where
    E: Error<RustTokens>,
    S: 'a,
{
    assert!(
        !punct.is_empty(),
        "Invalid empty punctuation for Rust proc-macro"
    );

    let mut puncts = punct
        .chars()
        .map(|c| Punct::new(c, Spacing::Joint))
        .collect::<Vec<_>>();

    let last = puncts.pop().unwrap();

    any::<RustTokens, _, _>()
        .repeated()
        .exactly(puncts.len())
        .slice()
        .try_map(move |toks, span| {
            toks.iter()
                .enumerate()
                .map(|(idx, tok)| {
                    tok.as_punct()
                        .ok_or_else(|| tok.clone())
                        .and_then(|punct| {
                            if punct_eq(punct, &puncts[idx]) {
                                Ok(punct.clone())
                            } else {
                                Err(RustToken::Punct(punct.clone()))
                            }
                        })
                        .map_err(|tok| E::expected_found([], Some(tok), span))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .chain(any::<RustTokens, _, _>().try_map(move |tok, span| {
            tok.into_punct()
                .and_then(|punct| {
                    if punct.as_char() == last.as_char() {
                        Ok(punct)
                    } else {
                        Err(RustToken::Punct(punct))
                    }
                })
                .map_err(|tok| E::expected_found([], Some(tok), span))
        }))
}
