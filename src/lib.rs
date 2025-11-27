#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]

/// Boolean extension traits for converting to [`Result`] and [`Option`] types.
pub mod bool;

/// Extension traits for panic unwrapping [`Result`] and [`Option`] types.
pub mod expect;

/// Extension traits for converting to [`Result`] and [`Option`] types.
pub mod into;

/// Extension traits for nested [`Result`]s.
pub mod nested;

/// Extension traits for [`Result`] and [`Option`] types.
pub mod sink;
