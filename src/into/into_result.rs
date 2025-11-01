/// Provides postfix conversion of any value into a [Result].
pub trait IntoResult {
    /// Moves a value into an [Ok] variant of a [Result].
    ///
    /// When used in terminal position, `E` can be inferred.
    /// When used in other positions, `E` can be specified.
    ///
    /// # Type Parameters
    /// - `E`: The error type to use in the [Result].
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::IntoResult;
    ///
    /// let owned_result: Result<u32, ()> = 42.into_ok();
    /// assert!(owned_result.is_ok());
    /// let owned_result = 42.into_ok::<()>();
    /// assert!(owned_result.is_ok());
    fn into_ok<E>(self) -> Result<Self, E>
    where
        Self: Sized,
    {
        Ok(self)
    }

    /// Move a error value into an [Err] variant of a [Result].
    ///
    /// When used in terminal position, `T` can be inferred.
    /// When used in other positions, `T` can be specified.
    ///
    /// # Type Parameters
    /// - `T`: The success type to use in the [Result].
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::IntoResult;
    ///
    /// let err = "error".into_err::<()>();
    /// assert!(err.is_err());
    /// let err: Result<(), &str> = "error".into_err();
    /// assert!(err.is_err());
    /// ```
    fn into_err<T>(self) -> Result<T, Self>
    where
        Self: Sized,
    {
        Err(self)
    }
}

/// Implements `IntoResult<T>` for all types.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
impl<T> IntoResult for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_ok() {
        let ok: Result<u32, ()> = 42.into_ok();
        assert_eq!(ok, Ok(42));
    }

    #[test]
    fn into_err() {
        let err = "error".into_err::<()>();
        assert_eq!(err, Err("error"));
    }
}
