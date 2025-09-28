/// Extension trait for transforming `Result<T, E>` into `Option<T>` by logging the [Err] variant with [tracing].
pub trait OkLog: crate::internal::Sealed {
    /// The type of the success value.
    type Success;

    /// The type of the error value.
    type Error;

    /// Transform `Result<T, E>` into `Option<T>` by logging the [Debug] value of [Err] variants at 'Level::ERROR'.
    ///
    /// # Example
    /// ```rust
    /// use result_utils::log::OkLog;
    ///
    /// let result: Result<u32, &str> = Ok(42);
    /// let some = result.ok_log();
    /// assert!(some.is_some());
    fn ok_log(self) -> Option<Self::Success>
    where
        Self::Error: std::fmt::Debug;
}

impl<T, E> OkLog for Result<T, E> {
    type Success = T;
    type Error = E;

    fn ok_log(self) -> Option<Self::Success>
    where
        E: std::fmt::Debug,
    {
        self.inspect_err(|e| tracing::error!(err = ?e)).ok()
    }
}
