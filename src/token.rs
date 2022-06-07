use std::hash::{Hash, Hasher};
use proc_macro2::{Literal, Ident, Punct, Delimiter, Spacing};

use crate::RustSpan;

/// A Rust Token - The flattened form of a [`TokenTree`][proc_macro2::TokenTree] with groups
/// converted into start and end delimiters.
#[derive(Clone, Debug)]
pub enum RustToken {
    /// A literal - Something like `1` or `"foo"`
    Literal(Literal),
    /// An identifier
    Ident(Ident),
    /// Punctuation, such as `+` or `#`
    Punct(Punct),
    /// A start delimiter for a group. All start delimiters have matching end delimiters due to
    /// Rust's macro parsing rules.
    StartDelim(Delimiter),
    /// An end delimiter for a group. All end delimiters have matching start delimiters due to
    /// Rust's macro parsing rules.
    EndDelim(Delimiter),
}

impl RustToken {
    /// A utility for passing to `filter_map` which converts tokens to a `Literal` or returns an
    /// error
    pub fn filter_literal<E: chumsky::Error<RustToken, Span = RustSpan>>(span: RustSpan, this: Self) -> Result<Literal, E> {
        if let RustToken::Literal(lit) = this {
            Ok(lit)
        } else {
            Err(E::expected_input_found(span, [], Some(this)))
        }
    }

    /// A utility for passing to `filter_map` which converts tokens to an `Ident` or returns an
    /// error
    pub fn filter_ident<E: chumsky::Error<RustToken, Span = RustSpan>>(span: RustSpan, this: Self) -> Result<Ident, E> {
        if let RustToken::Ident(ident) = this {
            Ok(ident)
        } else {
            Err(E::expected_input_found(span, [], Some(this)))
        }
    }

    /// A utility for passing to `filter_map` which converts tokens to a `Punct` or returns an
    /// error
    pub fn filter_punct<E: chumsky::Error<RustToken, Span = RustSpan>>(span: RustSpan, this: Self) -> Result<Punct, E> {
        if let RustToken::Punct(punct) = this {
            Ok(punct)
        } else {
            Err(E::expected_input_found(span, [], Some(this)))
        }
    }

    /// Returns whether this token is a Literal
    pub fn is_literal(&self) -> bool {
        matches!(self, RustToken::Literal(_))
    }

    /// Returns whether this token is an identifier
    pub fn is_ident(&self) -> bool {
        matches!(self, RustToken::Ident(_))
    }

    /// Returns whether this token is punctuation
    pub fn is_punct(&self) -> bool {
        matches!(self, RustToken::Punct(_))
    }

    /// Returns whether this token is a delimiter - start or end
    pub fn is_delim(&self) -> bool {
        self.is_start_delim() || self.is_end_delim()
    }

    /// Returns whether this token is a starting delimiter
    pub fn is_start_delim(&self) -> bool {
        matches!(self, RustToken::StartDelim(_))
    }

    /// Returns whether this token is an ending delimiter
    pub fn is_end_delim(&self) -> bool {
        matches!(self, RustToken::EndDelim(_))
    }
}

impl PartialEq for RustToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RustToken::Literal(this), RustToken::Literal(other)) => {
                // This seems sufficient - literals preserve their text into to_string well
                this.to_string() == other.to_string()
            }
            (RustToken::Ident(this), RustToken::Ident(other)) => {
                this == other
            }
            (RustToken::Punct(this), RustToken::Punct(other)) => {
                // to_string would lose spacing info
                this.as_char() == other.as_char() && this.spacing() == other.spacing()
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

impl PartialEq<Literal> for RustToken {
    fn eq(&self, other: &Literal) -> bool {
        if let RustToken::Literal(lit) = self {
            lit.to_string() == other.to_string()
        } else {
            false
        }
    }
}

impl PartialEq<Ident> for RustToken {
    fn eq(&self, other: &Ident) -> bool {
        if let RustToken::Ident(ident) = self {
            ident == other
        } else {
            false
        }
    }
}

impl PartialEq<Punct> for RustToken {
    fn eq(&self, other: &Punct) -> bool {
        if let RustToken::Punct(punct) = self {
            punct.as_char() == other.as_char() && punct.spacing() == other.spacing()
        } else {
            false
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
                punct.as_char().hash(state);
                match punct.spacing() {
                    Spacing::Alone => state.write_u8(0),
                    Spacing::Joint => state.write_u8(1),
                }
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
