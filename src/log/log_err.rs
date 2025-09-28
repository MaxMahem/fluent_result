/// A trait for handling [Err] variants of [UnitResult]s (Result<(), E>) by logging them with [tracing].
pub trait LogErr: crate::internal::Sealed {
    /// The error type.
    type Error;

    /// Handles [Err] variant by logging the [Debug] representation at `Level::ERROR``.
    ///
    /// # Example
    /// ```rust
    /// use result_utils::log::LogErr;
    ///
    /// Err("oops").log_err(); // Logs at ERROR level
    /// ```
    fn log_err(self)
    where
        Self::Error: std::fmt::Debug;

    /// Handles [Err] variants by emitting `msg` at `Level::ERROR`.
    ///
    /// # Examples
    /// ```rust
    /// use result_utils::log::LogErr;
    ///
    /// Err("oops").log_err_msg("Something went wrong");
    /// ```
    fn log_err_msg<D>(self, msg: D)
    where
        D: std::fmt::Display;
}

/// Implementation for all [UnitResult<E>].
impl<E> LogErr for crate::UnitResult<E> {
    type Error = E;

    fn log_err(self)
    where
        E: std::fmt::Debug,
    {
        _ = self.inspect_err(|e| tracing::error!(error = ?e));
    }

    fn log_err_msg<D>(self, msg: D)
    where
        D: std::fmt::Display,
    {
        _ = self.inspect_err(|_| tracing::error!(%msg));
    }
}
