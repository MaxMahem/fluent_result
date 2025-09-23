/// A trait that converts a value into a `Result::Err`.
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

/// Implements `IntoErr<T>` for all types.
///
/// # Type Parameters
/// - `E`: The error type.
impl<E> IntoErr for E {}
