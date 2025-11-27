#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]


/// Boolean extension traits for converting to [`Result`] and [`Option`] types.
pub mod bool;
mod expect_none;
mod into;
mod nested;
mod sink;
mod unwrap_never;

pub use bool::Then;
pub use expect_none::ExpectNone;
pub use into::{IntoOption, IntoResult};
pub use nested::{BoxErr, FlattenErr};
pub use sink::{SinkOption, SinkResult};
pub use unwrap_never::UnwrapNever;

/// A [Result] type with a unit `()` success type and variable error type.
///
/// Useful for failable methods that have no specific return value.
pub type UnitResult<E> = Result<(), E>;

/// An infallible [Result] type. That is a [Result] type that cannot fail.
pub type InfallibleResult<T> = Result<T, std::convert::Infallible>;

mod internal;
