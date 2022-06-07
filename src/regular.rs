
use std::ops::Range;
use proc_macro2::{Span, TokenStream};
use chumsky::Stream;

use super::{into_vec, RustSpan, RustToken};

impl chumsky::Span for RustSpan {
    type Context = ();
    type Offset = RustSpan;

    fn new(_: Self::Context, range: Range<Self::Offset>) -> Self {
        range.start.0.join(range.end.0)
            .unwrap()
            .into()
    }

    fn context(&self) -> Self::Context {
        ()
    }

    fn start(&self) -> Self::Offset {
        self.clone()
    }

    fn end(&self) -> Self::Offset {
        self.clone()
    }
}

pub fn stream_from_tokens(stream: TokenStream) -> Stream<'static, RustToken, RustSpan, impl Iterator<Item = (RustToken, RustSpan)>> {
    let tokens = into_vec(stream);

    Stream::from_iter(
        Span::call_site().into(),
        tokens.into_iter(),
    )
}
