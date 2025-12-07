use std::convert::Infallible;

/// A trait for unwrapping the [`Ok`] varaint of an [`Result<T, Infallible>`], panic free.
///
/// # Type Parameters
/// - `T`: The success type.
#[sealed::sealed]
pub trait UnwrapNever<T> {
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
    /// use fluent_result::expect::UnwrapNever;
    ///
    /// let result: Result<u32, Infallible> = Ok(42);
    /// let value = result.unwrap_never();
    /// assert_eq!(value, 42);
    /// ```
    fn unwrap_never(self) -> T;
}

/// Implementation for all [Result<T, Infallible>].
#[sealed::sealed]
impl<T> UnwrapNever<T> for Result<T, Infallible> {
    #[inline]
    fn unwrap_never(self) -> T {
        unsafe { self.unwrap_unchecked() }
    }
}
