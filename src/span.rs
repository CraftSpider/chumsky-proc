use proc_macro2::Span;
use std::borrow::Borrow;
use std::ops::Deref;

/// A wrapper around a Rust `Span`
#[derive(Clone, Debug)]
pub struct RustSpan(Span);

impl RustSpan {
    pub(crate) fn join(&self, other: RustSpan) -> Option<RustSpan> {
        self.0.join(other.0).map(RustSpan::from)
    }
}

impl From<Span> for RustSpan {
    fn from(span: Span) -> Self {
        RustSpan(span)
    }
}

impl Deref for RustSpan {
    type Target = Span;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Span> for RustSpan {
    fn as_ref(&self) -> &Span {
        &self.0
    }
}

impl Borrow<Span> for RustSpan {
    fn borrow(&self) -> &Span {
        &self.0
    }
}
