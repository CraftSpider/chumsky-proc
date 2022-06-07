
use std::ops::Range;
use proc_macro2::{Span, TokenStream};
use chumsky::Stream;

use super::{RustSpan, RustToken};
use crate::utils::into_vec;

impl chumsky::Span for RustSpan {
    type Context = ();
    type Offset = RustSpan;

    fn new(_: Self::Context, range: Range<Self::Offset>) -> Self {
        range.start.join(range.end).unwrap_or_else(|| Span::mixed_site().into())
    }

    fn context(&self) -> Self::Context {}

    fn start(&self) -> Self::Offset {
        self.clone()
    }

    fn end(&self) -> Self::Offset {
        self.clone()
    }
}

/// Generate a chumsky `Stream` from a Rust `TokenStream`
pub fn stream_from_tokens(stream: TokenStream) -> Stream<'static, RustToken, RustSpan, impl Iterator<Item = (RustToken, RustSpan)>> {
    let tokens = into_vec(stream);

    Stream::from_iter(
        Span::mixed_site().into(),
        tokens.into_iter(),
    )
}
