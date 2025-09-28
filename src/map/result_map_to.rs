use crate::internal;

/// Extension trait for transforming `Result<T, E>` into `Result<U, E>`
pub trait ResultMapTo: internal::Sealed {
    /// The error type.
    type Error;

    /// Maps `Result<T, E>` into `Result<U, E>` by substituting `Ok(other)` for the [Err] variant.
    ///
    /// Analogous to [Result::map(|_| other)](Result::map), but with a fixed value.
    ///
    /// # Example
    ///```rust
    /// use result_utils::map::ResultMapTo;
    ///
    /// let result: Result<bool, ()> = Ok(42).map_to(true);
    /// assert_eq!(result, Ok(true));
    /// ```
    fn map_to<TOther>(self, other: TOther) -> Result<TOther, Self::Error>;
}

/// Implementation for all `Result<T, E>`.
impl<T, E> ResultMapTo for Result<T, E> {
    type Error = E;

    fn map_to<TOther>(self, other: TOther) -> Result<TOther, Self::Error> {
        self.map(|_| other)
    }
}
