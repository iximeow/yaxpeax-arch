//! utilities to validate that implementations of traits in `yaxpeax-arch` uphold requirements
//! described in this crate.
//!
//! currently, this only includes tools to validate correct use of
//! [`yaxpeax_arch::display::DisplaySink`], but may grow in the future.

#[cfg(feature="alloc")]
mod display;
#[cfg(feature="alloc")]
pub use display::{DisplaySinkValidator, DisplaySinkWriteComparator};
