use proc_macro::{Ident, Punct, Spacing, Span, TokenStream};
use std::panic::{AssertUnwindSafe, catch_unwind, set_hook};
use std::io::Write;

#[proc_macro]
pub fn proc(_: TokenStream) -> TokenStream {
    let mut out = std::fs::File::create("proc.output")
        .unwrap();

    for c in 0..(char::MAX as u32) {
        if let Some(c) = char::from_u32(c) {
            set_hook(Box::new(|_| ()));
            let _ = catch_unwind(AssertUnwindSafe(|| {
                let p = Punct::new(c, Spacing::Alone);
                let _ = writeln!(out, "Valid Punct: {}", p);
            }));
            let _ = catch_unwind(AssertUnwindSafe(|| {
                let i = Ident::new(&c.to_string(), Span::call_site());
                let _ = writeln!(out, "Valid Ident: {}", i);
            }));
        }
    }
    TokenStream::new()
}