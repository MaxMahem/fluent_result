use crate::internal;

/// Provies postifx helpers for transforming a [Result<T, E>] into [Result<U, E>]
pub trait ResultMapTo: internal::Sealed {
    /// The success type.
    type Success;
    /// The error type.
    type Error;

    /// Maps [Result<T, E>] into [Result<U, E>] by substituting Ok(other)` for the [Err] variant.
    ///
    /// Analogous to [Result::map(|_| other)](Result::map), but with a fixed value.
    ///
    /// # Type Parameters
    /// - `U`: The new success type.
    ///
    /// # Arguments
    /// - `other`: The value to substitute for the [Err] variant.
    ///
    /// # Example
    ///```rust
    /// use fluent_result::ResultMapTo;
    ///
    /// let result: Result<bool, ()> = Ok(42).map_to(true);
    /// assert_eq!(result, Ok(true));
    /// ```
    fn map_to<TOther>(self, other: TOther) -> Result<TOther, Self::Error>;

    /// Maps [Result<T, E1>] into [Result<T, E2>] by substituting `Err(other)` for the [Err] variant.
    ///
    /// Analogous to [Result::map_err(|_| other)](Result::map_err), but with a fixed value.
    ///
    /// # Type Parameters
    /// - `EOther`: The new error type.
    ///
    /// # Arguments
    /// - `other`: The value to substitute for the [Err] variant.
    ///
    /// # Example
    ///```rust
    /// use fluent_result::ResultMapTo;
    ///
    /// let result: Result<(), bool> = Err(42).map_err_to(false);
    /// assert_eq!(result, Err(false));
    /// ```
    fn map_err_to<EOther>(self, other: EOther) -> Result<Self::Success, EOther>;
}

/// Implementation for all `Result<T, E>`.
impl<T, E> ResultMapTo for Result<T, E> {
    type Success = T;
    type Error = E;

    fn map_to<TOther>(self, other: TOther) -> Result<TOther, Self::Error> {
        self.map(|_| other)
    }

    fn map_err_to<EOther>(self, other: EOther) -> Result<Self::Success, EOther> {
        self.map_err(|_| other)
    }
}
