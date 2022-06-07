use chumsky::prelude::*;
use chumsky_proc::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Punct};

#[derive(Debug)]
struct SimpleExpr {
    left: Ident,
    middle: Punct,
    right: Ident,
}

fn parser() -> impl Parser<RustToken, SimpleExpr, Error = Simple<RustToken, RustSpan>> {
    filter_map(RustToken::filter_ident)
        .then(filter_map(RustToken::filter_punct))
        .then(filter_map(RustToken::filter_ident))
        .map(|((left, middle), right)| SimpleExpr {
            left,
            middle,
            right,
        })
}

#[proc_macro]
pub fn example(item: TokenStream) -> TokenStream {
    println!("{:?}", parser().parse(stream_from_tokens(item.into())),);

    TokenStream::new()
}
