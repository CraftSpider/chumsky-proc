//! Implementations for standard chumsky traits and types

use std::ops::{Range, RangeFrom};
use chumsky::zero_copy::input::{Input, SliceInput};
use proc_macro2::{Span, TokenStream, TokenTree};

use super::{RustSpan, RustToken};
use crate::utils::into_vec;

pub struct RustTokens(Vec<RustToken>, Vec<RustSpan>);

impl RustTokens {
    pub fn new(ts: TokenStream) -> RustTokens {
        let (tokens, spans) = into_vec(ts).into_iter().unzip();
        RustTokens(tokens, spans)
    }
}

impl Input for RustTokens {
    type Offset = usize;
    type Token = RustToken;
    type Span = RustSpan;

    fn start(&self) -> Self::Offset {
        0
    }

    fn next(&self, offset: Self::Offset) -> (Self::Offset, Option<Self::Token>) {
        if let Some(tok) = self.0.get(offset) {
            (offset + 1, Some(tok.clone()))
        } else {
            (offset, None)
        }
    }

    fn span(&self, range: Range<Self::Offset>) -> Self::Span {
        self.1
            .get(range)
            .and_then(|toks| {
                toks.iter()
                    .fold(None::<RustSpan>, |acc, span| {
                        if let Some(prev) = acc {
                            prev.join(*span)
                        } else {
                            Some(*span)
                        }
                    })
            })
            .unwrap_or_else(|| RustSpan::from(Span::call_site()))
    }
}

impl SliceInput for RustTokens {
    type Slice = [RustToken];

    fn slice(&self, range: Range<Self::Offset>) -> &Self::Slice {
        &self.0[range]
    }

    fn slice_from(&self, from: RangeFrom<Self::Offset>) -> &Self::Slice {
        &self.0[from]
    }
}
