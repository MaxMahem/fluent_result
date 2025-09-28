/// A trait for logging the [Debug] value of the [Ok] variant of [Result] with [tracing].
pub trait InspectLog: crate::internal::Sealed {
    type Success;

    /// Logs the [Debug] value of the [Ok] variant of [Result] at 'Level::Info'.
    ///
    /// # Example
    /// ```rust
    /// use result_utils::log::InspectLog;
    ///
    /// let result: Result<u32, ()> = Ok(42).inspect_log();
    /// ```
    fn inspect_log(self) -> Self
    where
        Self::Success: std::fmt::Debug;

    /// Logs the [Debug] value of the [Ok] variant of [Result] at 'Level::Debug', on debug mode only.
    ///
    /// # Example
    /// ```rust
    /// use result_utils::log::InspectLog;
    ///
    /// let result: Result<u32, ()> = Ok(42).inspect_log_dbg();
    /// ```
    fn inspect_log_dbg(self) -> Self
    where
        Self::Success: std::fmt::Debug;
}

/// Implementation for all `Result<T, E>`.
impl<T, E> InspectLog for Result<T, E> {
    type Success = T;

    fn inspect_log(self) -> Self
    where
        T: std::fmt::Debug,
    {
        self.inspect(|t| tracing::info!(?t))
    }

    fn inspect_log_dbg(self) -> Self
    where
        Self::Success: std::fmt::Debug,
    {
        match cfg!(debug_assertions) {
            true => self.inspect(|t| tracing::debug!(?t)),
            false => self,
        }
    }
}
