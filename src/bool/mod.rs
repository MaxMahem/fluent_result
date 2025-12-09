//! Extension traits for [`bool`].

/// [`assert!`] methods for expected [`bool`] values.
pub mod expect;
mod then;

pub use then::Then;
