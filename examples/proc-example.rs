use chumsky::zero_copy::prelude::*;
use chumsky_proc::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Punct};
use chumsky_proc::RustTokens;

#[derive(Debug)]
struct SimpleExpr {
    left: Ident,
    middle: Punct,
    right: Ident,
}

fn parser<'a>() -> impl Parser<'a, RustTokens, SimpleExpr, Simple<RustTokens>, ()> {
    any().try_map(RustToken::filter_ident)
        .then(any().try_map(RustToken::filter_punct))
        .then(any().try_map(RustToken::filter_ident))
        .map(|((left, middle), right)| SimpleExpr {
            left,
            middle,
            right,
        })
}

#[proc_macro]
pub fn example(item: TokenStream) -> TokenStream {
    let toks = RustTokens::new(item.into());
    println!("{:?}", parser().parse(&toks),);

    TokenStream::new()
}
