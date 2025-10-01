use crate::log::Level;

/// Provides a postfix method to passthrough and log the [Debug] value [Result] variants with [tracing].
pub trait TapLog: crate::internal::Sealed {
    /// The error type of the [Result].
    type Error;
    /// The success type of the [Result].
    type Success;

    /// Logs the [Debug] value of the [Ok] variant of [Result] (if any) at the specified `tracing::Level`
    /// with a contextual message (`ctx`). If `ctx` is empty, then it is omitted. Returns self unchanged.
    ///
    /// This method is useful for logging values in method chains or pipelines without disrupting flow.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::log::{TapLog, Level};
    ///
    /// let result: Result<u32, u32> = Ok(42).tap_ok_log(Level::INFO, "hello"); // logs "INFO ctx=hello ok=42"
    /// let result: Result<u32, u32> = Ok(42).tap_ok_log(Level::INFO, "");      // logs "INFO ok=42"
    /// let result: Result<u32, u32> = Err(42).tap_ok_log(Level::INFO, "");     // logs nothing
    /// ```
    fn tap_ok_log(self, level: tracing::Level, ctx: &str) -> Self
    where
        Self::Success: std::fmt::Debug;

    /// Logs the [Debug] value of the [Err] variant of [Result] (if any) at the specified `tracing::Level`
    /// with a contextual message (`ctx`). If `ctx` is empty, then it is omitted. Returns self unchanged.
    ///
    /// This method is useful for logging values in method chains or pipelines without disrupting flow.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::log::{TapLog, Level};
    ///
    /// let result: Result<u32, u32> = Err(42).tap_err_log(Level::INFO, "hello"); // logs "INFO ctx=hello err=42"
    /// let result: Result<u32, u32> = Err(42).tap_err_log(Level::INFO, "");      // logs "INFO err=42"
    /// let result: Result<u32, u32> = Ok(42).tap_err_log(Level::INFO, "");       // logs nothing
    /// ```
    fn tap_err_log(self, level: tracing::Level, ctx: &str) -> Self
    where
        Self::Error: std::fmt::Debug;
}

impl<T, E> TapLog for Result<T, E> {
    type Error = E;
    type Success = T;

    fn tap_ok_log(self, level: tracing::Level, ctx: &str) -> Self
    where
        T: std::fmt::Debug,
    {
        match (ctx, level) {
            ("", Level::ERROR) => self.inspect(|t| tracing::error!(ok = ?t)),
            ("", Level::WARN) => self.inspect(|t| tracing::warn!(ok = ?t)),
            ("", Level::INFO) => self.inspect(|t| tracing::info!(ok = ?t)),
            ("", Level::DEBUG) => self.inspect(|t| tracing::debug!(ok = ?t)),
            ("", Level::TRACE) => self.inspect(|t| tracing::trace!(ok = ?t)),

            (ctx, Level::ERROR) => self.inspect(|t| tracing::error!(%ctx, ok = ?t)),
            (ctx, Level::WARN) => self.inspect(|t| tracing::warn!(%ctx, ok = ?t)),
            (ctx, Level::INFO) => self.inspect(|t| tracing::info!(%ctx, ok = ?t)),
            (ctx, Level::DEBUG) => self.inspect(|t| tracing::debug!(%ctx, ok = ?t)),
            (ctx, Level::TRACE) => self.inspect(|t| tracing::trace!(%ctx, ok = ?t)),
        }
    }

    fn tap_err_log(self, level: tracing::Level, ctx: &str) -> Self
    where
        E: std::fmt::Debug,
    {
        match (ctx, level) {
            ("", Level::ERROR) => self.inspect_err(|e| tracing::error!(err = ?e)),
            ("", Level::WARN) => self.inspect_err(|e| tracing::warn!(err = ?e)),
            ("", Level::INFO) => self.inspect_err(|e| tracing::info!(err = ?e)),
            ("", Level::DEBUG) => self.inspect_err(|e| tracing::debug!(err = ?e)),
            ("", Level::TRACE) => self.inspect_err(|e| tracing::trace!(err = ?e)),

            (ctx, Level::ERROR) => self.inspect_err(|e| tracing::error!(%ctx, err = ?e)),
            (ctx, Level::WARN) => self.inspect_err(|e| tracing::warn!(%ctx, err = ?e)),
            (ctx, Level::INFO) => self.inspect_err(|e| tracing::info!(%ctx, err = ?e)),
            (ctx, Level::DEBUG) => self.inspect_err(|e| tracing::debug!(%ctx, err = ?e)),
            (ctx, Level::TRACE) => self.inspect_err(|e| tracing::trace!(%ctx, err = ?e)),
        }
    }
}
