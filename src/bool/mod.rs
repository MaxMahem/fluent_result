//! Extension traits for [`bool`].

/// [`assert!`] methods for expected [`bool`] values.
pub mod expect;
mod then;

pub use expect::{dbg, rls};
pub use then::Then;
