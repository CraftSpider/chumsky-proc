//! Implementation of a wrapper for a Rust `Span` to avoid foreign impl issues

use chumsky::zero_copy::span::Span as ChumskySpan;
use proc_macro2::Span;
use std::borrow::Borrow;
use std::ops::Deref;

/// A wrapper around a Rust `Span`
#[derive(Copy, Clone, Debug)]
pub struct RustSpan(Span);

impl RustSpan {
    /// Join this span to another - always returns `None` when not on nightly
    pub(crate) fn join(self, other: RustSpan) -> Option<RustSpan> {
        self.0.join(other.0).map(RustSpan::from)
    }
}

impl From<Span> for RustSpan {
    fn from(span: Span) -> Self {
        RustSpan(span)
    }
}

impl From<RustSpan> for Span {
    fn from(span: RustSpan) -> Self {
        span.0
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

impl ChumskySpan for RustSpan {
    type Context = ();
    type Offset = RustSpan;
}
