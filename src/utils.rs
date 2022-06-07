use proc_macro2::{Punct, TokenStream, TokenTree};

use super::{RustSpan, RustToken};

pub fn into_vec(stream: TokenStream) -> Vec<(RustToken, RustSpan)> {
    stream
        .into_iter()
        .flat_map(|tree| match tree {
            TokenTree::Group(group) => {
                let inner = into_vec(group.stream());
                let mut out = Vec::with_capacity(inner.len() + 2);
                out.push((
                    RustToken::StartDelim(group.delimiter()),
                    group.span_open().into(),
                ));
                out.extend(inner);
                out.push((
                    RustToken::EndDelim(group.delimiter()),
                    group.span_close().into(),
                ));
                out
            }
            TokenTree::Ident(ident) => {
                let span = ident.span().into();
                vec![(RustToken::Ident(ident), span)]
            }
            TokenTree::Punct(punct) => {
                let span = punct.span().into();
                vec![(RustToken::Punct(punct), span)]
            }
            TokenTree::Literal(lit) => {
                let span = lit.span().into();
                vec![(RustToken::Literal(lit), span)]
            }
        })
        .collect()
}

pub fn punct_eq(left: &Punct, right: &Punct) -> bool {
    // to_string would lose spacing info
    left.as_char() == right.as_char() && left.spacing() == right.spacing()
}
