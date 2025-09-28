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
    /// Err("oops").log_err(tracing::Level::ERROR); // Logs at ERROR level
    /// ```
    fn log_err(self, level: tracing::Level)
    where
        Self::Error: std::fmt::Debug;
}

/// Implementation for all [UnitResult<E>].
impl<E> LogErr for crate::UnitResult<E> {
    type Error = E;

    fn log_err(self, level: tracing::Level)
    where
        E: std::fmt::Debug,
    {
        match level {
            tracing::Level::ERROR => _ = self.inspect_err(|e| tracing::error!(error = ?e)),
            tracing::Level::WARN => _ = self.inspect_err(|e| tracing::warn!(error = ?e)),
            tracing::Level::INFO => _ = self.inspect_err(|e| tracing::info!(error = ?e)),
            tracing::Level::DEBUG => _ = self.inspect_err(|e| tracing::debug!(error = ?e)),
            tracing::Level::TRACE => _ = self.inspect_err(|e| tracing::trace!(error = ?e)),
        }
    }
}
