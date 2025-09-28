/// A trait for converting a [Result] into a [Result] with a different error type.
pub trait OrErr {
    /// The type of the value when the result is `Ok`.
    type Ok;

    /// Convert a [Result] into a [Result] with a different [Err] type.
    ///
    /// # Type Parameters
    /// - `EOut`: The new error type.
    ///
    /// # Parameters
    /// - `err`: The new error value.
    ///
    /// # Returns
    /// A [Result] with the new error type.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::OrErr;
    /// let result: Result<u32, ()> = Ok(42);
    /// let result: Result<u32, std::fmt::Error> = result.or_err(std::fmt::Error);
    /// assert!(result.is_err());
    /// ```
    fn or_err<E>(self, err: E) -> Result<Self::Ok, E>;
}

impl<T, E> OrErr for Result<T, E> {
    type Ok = T;

    fn or_err<EOut>(self, err: EOut) -> Result<T, EOut> {
        Err(err)
    }
}
