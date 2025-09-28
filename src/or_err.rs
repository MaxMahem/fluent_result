/// A trait for converting a [Result] into a [Result] with a different error type.
pub trait OrErr: crate::internal::Sealed {
    /// The type of the value when the result is `Ok`.
    type Ok;

    /// Convert a [Result] into a [Result] with a different [Err] type.
    ///
    /// # Type Parameters
    /// - `E`: The error type to use in the `Result`
    ///
    /// # Parameters
    /// - `err`: The error value to use in the `Result
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::OrErr;
    /// let result: Result<u32, ()> = Ok(42);
    /// let result: Result<u32, &str> = result.or_err("oops");
    /// assert!(result.is_err());
    /// ```
    fn or_err<E>(self, err: E) -> Result<Self::Ok, E>;
}

/// Implementation for all `Result<T, E>`.
impl<T, E> OrErr for Result<T, E> {
    type Ok = T;

    fn or_err<EOut>(self, err: EOut) -> Result<T, EOut> {
        Err(err)
    }
}
