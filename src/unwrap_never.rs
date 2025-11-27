use crate::InfallibleResult;

/// A trait for unwrapping the [`Ok`] varaint of an [`InfallibleResult`], panic free.
///
/// # Type Parameters
/// - `T`: The success type.
pub trait UnwrapNever<T>: crate::internal::Sealed {
    /// Unwraps a [`Result<T, Infallible>`] without the possibility of panic.
    /// Functionaly this is the same as calling [`Result::unwrap`].
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::convert::Infallible;
    /// use fluent_result::UnwrapNever;
    ///
    /// let result: Result<u32, Infallible> = Ok(42);
    /// let value = result.unwrap_never();
    /// assert_eq!(value, 42);
    /// ```
    fn unwrap_never(self) -> T;
}

/// Implementation for all [Result<T, Infallible>].
impl<T> UnwrapNever<T> for InfallibleResult<T> {
    fn unwrap_never(self) -> T {
        unsafe { self.unwrap_unchecked() }
    }
}
