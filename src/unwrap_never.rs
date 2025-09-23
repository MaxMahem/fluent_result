use std::convert::Infallible;

use crate::internal::Sealed;

/// A trait for unwrapping `Result<T, Infallible>` with better expressed intentionality.
/// Functionaly this is the same as calling `Result.unwrap()`, and never panics.
///
/// # Safety
///
/// Internally uses `unwrap_unchecked()` under the assumption that the
/// `Err` variant is uninhabited. Calling this on a `Result` that somehow
/// contains an `Err` will invoke undefined behavior.
pub trait UnwrapNever<T>: Sealed {
    /// Unwraps a `Result<T, Infallible>` without the possibility of panic.
    ///
    /// # Panics
    ///
    /// This method does not panic, but it invokes UB if the `Err` variant
    /// is somehow present, which should not be possible.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::convert::Infallible;
    /// # use result_utils::UnwrapNever;
    /// let result: Result<u32, Infallible> = Ok(42);
    /// let value = result.unwrap_never();
    /// assert_eq!(value, 42);
    /// ```
    fn unwrap_never(self) -> T;
}

impl<T> UnwrapNever<T> for Result<T, Infallible> {
    fn unwrap_never(self) -> T {
        unsafe { self.unwrap_unchecked() }
    }
}
