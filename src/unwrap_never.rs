/// A trait for unwrapping the [Ok] varaint of an [InfallibleResult] with better expressed intentionality.
pub trait UnwrapNever<T>: crate::internal::Sealed {
    /// Unwraps a `Result<T, Infallible>` without the possibility of panic.
    /// Functionaly this is the same as calling `Result.unwrap()`
    ///
    /// # Panics
    ///
    /// This method does not panic, but it invokes UB if the `Err` variant
    /// is somehow present, which should not be possible.
    ///
    /// # Safety
    ///
    /// Internally uses `unwrap_unchecked()` under the assumption that the
    /// `Err` variant is uninhabited. Calling this on a `InfallibleResult` that somehow
    /// contains an `Err` will invoke undefined behavior.
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

/// Implementation for all [Result<T, Infallible>].
impl<T> UnwrapNever<T> for crate::InfallibleResult<T> {
    fn unwrap_never(self) -> T {
        unsafe { self.unwrap_unchecked() }
    }
}
