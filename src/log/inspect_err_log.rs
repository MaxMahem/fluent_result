/// A trait for logging the [Debug] value of the [Err] variant of [Result] with [tracing].
pub trait InspectErrLog: crate::internal::Sealed {
    /// The error type of the [Result].
    type Error;

    /// Logs the [Debug] value of the [Err] variant of [Result] with [tracing].
    ///
    /// # Example
    /// ```rust
    /// use result_utils::log::InspectErrLog;
    ///
    /// let result: Result<u32, u32> = Err(42).inspect_err_log();
    fn inspect_err_log(self) -> Self
    where
        Self::Error: std::fmt::Debug;
}

impl<T, E> InspectErrLog for Result<T, E> {
    type Error = E;

    fn inspect_err_log(self) -> Self
    where
        E: std::fmt::Debug,
    {
        self.inspect_err(|e| tracing::error!(error = ?e))
    }
}
