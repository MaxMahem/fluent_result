/// A trait that converts a value into a `Result::Err`.
///
/// Note this is only implemented for owned types that implement `std::error::Error`, not all types.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
pub trait IntoErr<T> {
    /// Move a error value into a `Result::Err`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoErr;
    /// let err = std::fmt::Error;
    /// let result: Result<u8, std::fmt::Error> = err.into_err();
    /// assert!(result.is_err());
    /// ```
    fn into_err(self) -> Result<T, Self>
    where
        Self: Sized,
    {
        Err(self)
    }
}

/// Implements `IntoErr<T>` for any error type `E` that implements `std::error::Error`.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
/// - `E`: The error type, which must implement `std::error::Error`.
impl<T, E: std::error::Error> IntoErr<T> for E {}
