#![doc = include_str!("../README.md")]

mod handle_err;
mod into;
mod map;
mod unwrap_never;
mod unwrap_result;

#[cfg(feature = "tracing")]
/// Provides traits for transforming and manipulating [Result] and [Option] variants with `tracing`.
pub mod log;

pub use handle_err::HandleErr;
pub use into::{IntoOption, IntoResult};
pub use map::{OptionMapTo, ResultMapTo};
pub use unwrap_never::UnwrapNever;
pub use unwrap_result::UnwrapResult;

/// A [Result] type with a unit `()` success type and variable error type.
///
/// Useful for failable methods that have no specific return value.
pub type UnitResult<E> = Result<(), E>;

/// An infallible [Result] type. That is a [Result] type that cannot fail.
pub type InfallibleResult<T> = Result<T, std::convert::Infallible>;

mod internal;
