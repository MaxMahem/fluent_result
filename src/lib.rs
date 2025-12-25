#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
// lints
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(doc)]
extern crate std;

/// Extension traits for `bool` values.
pub mod bool;

/// Extension traits for panic unwrapping [`Result`] and [`Option`] types.
pub mod expect;

/// Extension traits for converting to [`Result`] and [`Option`] types.
pub mod into;

/// Extension traits for nested [`Result`]s.
pub mod nested;

/// Extension traits for [`Result`] and [`Option`] types.
pub mod sink;
