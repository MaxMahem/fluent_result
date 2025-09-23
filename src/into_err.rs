/// A trait that converts a value into a `Result::Err`.
///
/// Note this is only implemented for owned types that implement `std::error::Error`, not all types.
pub trait IntoErr {
    /// Move a error value into a `Result::Err`.
    ///
    /// When used in terminal position, `T` can be inferred.
    /// When used in other positions, `T` can be specified.
    ///
    /// # Type Parameters
    /// - `T`: The success type to use in the `Result`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoErr;
    /// let err = std::fmt::Error.into_err::<()>();
    /// assert!(err.is_err());
    /// let err: Result<(), std::fmt::Error> = std::fmt::Error.into_err();
    /// assert!(err.is_err());
    /// ```
    fn into_err<T>(self) -> Result<T, Self>
    where
        Self: Sized,
    {
        Err(self)
    }
}

/// Implements `IntoErr<T>` for any error type `E` that implements `std::error::Error`.
///
/// # Type Parameters
/// - `E`: The error type, which must implement `std::error::Error`.
impl<E: std::error::Error> IntoErr for E {}
