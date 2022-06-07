
use proc_macro::TokenStream;
use chumsky_proc::{RustToken, RustSpan, stream_from_tokens};
use chumsky::prelude::*;

#[derive(Debug)]
struct SimpleExpr {
    left: RustToken,
    middle: RustToken,
    right: RustToken,
}

fn parser() -> impl Parser<RustToken, SimpleExpr, Error = Simple<RustToken, RustSpan>> {
    filter(RustToken::is_ident)
        .then(filter(RustToken::is_punct))
        .then(filter(RustToken::is_ident))
        .map(|((left, middle), right)| SimpleExpr { left, middle, right })
}

#[proc_macro]
pub fn example(item: TokenStream) -> TokenStream {
    println!(
        "{:?}",
        parser().parse(stream_from_tokens(item.into())),
    );

    TokenStream::new()
}
