use std::ops::Range;
use proc_macro2::{Literal, Ident, Punct, Delimiter, TokenStream};
use chumsky::zero_copy::Input;

use super::RustToken;

pub struct RustTokens(Vec<RustToken>);

impl Input for RustTokens {
    type Offset = ();
    type Token = ();
    type Span = ();

    fn start(&self) -> Self::Offset {
        todo!()
    }

    fn next(&self, offset: Self::Offset) -> (Self::Offset, Option<Self::Token>) {
        todo!()
    }

    fn span(&self, range: Range<Self::Offset>) -> Self::Span {
        todo!()
    }
}

pub fn input_from_tokens(stream: TokenStream) -> RustTokens {
    todo!()
}
