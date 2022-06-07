use std::hash::{Hash, Hasher};
use proc_macro2::{TokenTree, TokenStream, Literal, Ident, Punct, Span, Delimiter};

mod regular;
#[cfg(feature = "zero_copy")]
mod zero_copy;

pub use regular::*;
#[cfg(feature = "zero_copy")]
pub use zero_copy::*;

#[derive(Clone, Debug)]
pub struct RustSpan(Span);

impl From<Span> for RustSpan {
    fn from(span: Span) -> Self {
        RustSpan(span)
    }
}

#[derive(Clone, Debug)]
pub enum RustToken {
    Literal(Literal),
    Ident(Ident),
    Punct(Punct),
    StartDelim(Delimiter),
    EndDelim(Delimiter),
}

impl PartialEq for RustToken {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Improve eq for literal and punct
        match (self, other) {
            (RustToken::Literal(this), RustToken::Literal(other)) => {
                this.to_string() == other.to_string()
            }
            (RustToken::Ident(this), RustToken::Ident(other)) => {
                this == other
            }
            (RustToken::Punct(this), RustToken::Punct(other)) => {
                this.to_string() == other.to_string()
            }
            (RustToken::StartDelim(this), RustToken::StartDelim(other)) => {
                this == other
            }
            (RustToken::EndDelim(this), RustToken::EndDelim(other)) => {
                this == other
            }
            _ => false,
        }
    }
}

impl Eq for RustToken {}

impl Hash for RustToken {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RustToken::Literal(lit) => {
                state.write_u8(0);
                lit.to_string().hash(state);
            }
            RustToken::Ident(ident) => {
                state.write_u8(1);
                ident.hash(state);
            }
            RustToken::Punct(punct) => {
                state.write_u8(2);
                punct.to_string().hash(state);
            }
            RustToken::StartDelim(delim) => {
                state.write_u8(3);
                match delim {
                    Delimiter::Parenthesis => state.write_u8(0),
                    Delimiter::Brace => state.write_u8(1),
                    Delimiter::Bracket => state.write_u8(2),
                    Delimiter::None => state.write_u8(3),
                }
            }
            RustToken::EndDelim(delim) => {
                state.write_u8(4);
                match delim {
                    Delimiter::Parenthesis => state.write_u8(0),
                    Delimiter::Brace => state.write_u8(1),
                    Delimiter::Bracket => state.write_u8(2),
                    Delimiter::None => state.write_u8(3),
                }
            }
        }
    }
}

impl RustToken {
    pub fn is_literal(&self) -> bool {
        matches!(self, RustToken::Literal(_))
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, RustToken::Ident(_))
    }

    pub fn is_punct(&self) -> bool {
        matches!(self, RustToken::Punct(_))
    }

    pub fn is_start_delim(&self) -> bool {
        matches!(self, RustToken::StartDelim(_))
    }

    pub fn is_end_delim(&self) -> bool {
        matches!(self, RustToken::EndDelim(_))
    }
}

fn into_vec(stream: TokenStream) -> Vec<(RustToken, RustSpan)> {
    stream.into_iter()
        .flat_map(|tree| match tree {
            TokenTree::Group(group) => {
                let inner = into_vec(group.stream());
                let mut out = Vec::with_capacity(inner.len() + 2);
                out.push((RustToken::StartDelim(group.delimiter()), group.span_open().into()));
                out.extend(inner);
                out.push((RustToken::EndDelim(group.delimiter()), group.span_close().into()));
                out
            }
            TokenTree::Ident(ident) => {
                let span = ident.span().into();
                vec![(RustToken::Ident(ident), span)]
            },
            TokenTree::Punct(punct) => {
                let span = punct.span().into();
                vec![(RustToken::Punct(punct), span)]
            },
            TokenTree::Literal(lit) => {
                let span = lit.span().into();
                vec![(RustToken::Literal(lit), span)]
            },
        })
        .collect()
}
