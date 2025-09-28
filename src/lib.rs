pub mod handle_err;
pub mod into;
pub mod map;
pub mod unwrap_never;
pub mod unwrap_ok;

#[cfg(feature = "tracing")]
pub mod log;

pub use handle_err::HandleErr;
pub use unwrap_never::UnwrapNever;
pub use unwrap_ok::UnwrapOk;

/// An alias for result types with a unit result and variable error type.
///
/// Useful for failable methods that have no specific return value.
pub type UnitResult<E> = Result<(), E>;

/// An alias for infallible result types. That is a result type that cannot fail.
pub type InfallibleResult<T> = Result<T, std::convert::Infallible>;

mod internal {
    pub trait Sealed {}

    //impl<T> Sealed for Result<T, std::convert::Infallible> {}
    impl<T, E> Sealed for Result<T, E> {}
}
