use std::hash::{Hash, Hasher};
use proc_macro2::{Literal, Ident, Punct, Delimiter, Spacing};

use crate::RustSpan;
use crate::utils::punct_eq;

macro_rules! impl_items {
    ($variant:ident, $inner:ty, $name:literal, $is_name:ident, $as_name:ident, $into_name:ident) => {
        #[inline]
        #[doc = concat!("Returns whether this token is ", $name)]
        pub fn $is_name(&self) -> bool {
            matches!(self, RustToken::$variant(_))
        }

        #[inline]
        #[doc = concat!("Get this token as ", $name, ", or return `None`")]
        pub fn $as_name(&self) -> Option<&$inner> {
            if let Self::$variant(inner) = self {
                Some(inner)
            } else {
                None
            }
        }

        #[inline]
        #[doc = concat!("Convert this token into ", $name, ", or return `Err(self)`")]
        pub fn $into_name(self) -> Result<$inner, Self> {
            if let Self::$variant(inner) = self {
                Ok(inner)
            } else {
                Err(self)
            }
        }
    }
}

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
        this.into_literal()
            .map_err(|this| E::expected_input_found(span, [], Some(this)))
    }

    /// A utility for passing to `filter_map` which converts tokens to an `Ident` or returns an
    /// error
    pub fn filter_ident<E: chumsky::Error<RustToken, Span = RustSpan>>(span: RustSpan, this: Self) -> Result<Ident, E> {
        this.into_ident()
            .map_err(|this| E::expected_input_found(span, [], Some(this)))
    }

    /// A utility for passing to `filter_map` which converts tokens to a `Punct` or returns an
    /// error
    pub fn filter_punct<E: chumsky::Error<RustToken, Span = RustSpan>>(span: RustSpan, this: Self) -> Result<Punct, E> {
        this.into_punct()
            .map_err(|this| E::expected_input_found(span, [], Some(this)))
    }

    impl_items!(Literal, Literal, "a literal", is_literal, as_literal, into_literal);
    impl_items!(Ident, Ident, "an identifier", is_ident, as_ident, into_ident);
    impl_items!(Punct, Punct, "punctuation", is_punct, as_punct, into_punct);
    impl_items!(StartDelim, Delimiter, "a starting delimiter", is_start_delim, as_start_delim, into_start_delim);
    impl_items!(EndDelim, Delimiter, "an ending delimiter", is_end_delim, as_end_delim, into_end_delim);

    /// Returns whether this token is a delimiter - start or end
    pub fn is_delim(&self) -> bool {
        self.is_start_delim() || self.is_end_delim()
    }

    /// Get this token as a delimiter, start or end, or return `None`
    pub fn as_delim(&self) -> Option<&Delimiter> {
        self.as_start_delim().or_else(|| self.as_end_delim())
    }

    /// Convert this token into a delimiter, start or end, or return `Err(self)`
    pub fn into_delim(self) -> Result<Delimiter, RustToken> {
        self.into_start_delim().or_else(|this| this.into_end_delim())
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
                punct_eq(this, other)
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
        self.as_literal()
            .map(|lit| lit.to_string() == other.to_string())
            .unwrap_or(false)
    }
}

impl PartialEq<Ident> for RustToken {
    fn eq(&self, other: &Ident) -> bool {
        self.as_ident()
            .map(|ident| ident == other)
            .unwrap_or(false)
    }
}

impl PartialEq<Punct> for RustToken {
    fn eq(&self, other: &Punct) -> bool {
        self.as_punct()
            .map(|punct| punct_eq(punct, other))
            .unwrap_or(false)
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
