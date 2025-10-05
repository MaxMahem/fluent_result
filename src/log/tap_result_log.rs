use crate::log::Level;

/// Provides a postfix method to passthrough and log the [Debug] value [Result] variants with
/// [tracing].
pub trait TapResultLog: crate::internal::Sealed {
    /// The error type of the [Result].
    type Error;
    /// The success type of the [Result].
    type Success;

    /// Logs the [Debug] value of the [Ok] variant of [Result] (if any) at the specified
    /// `tracing::Level` with a contextual message (`ctx`). If `ctx` is empty, then it is omitted.
    /// Returns self unchanged.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::log::{TapResultLog, Level};
    ///
    /// let result: Result<u32, u32> = Ok(42).tap_ok_log(Level::INFO, "hello"); // logs "INFO ctx=hello ok=42"
    /// let result: Result<u32, u32> = Ok(42).tap_ok_log(Level::INFO, "");      // logs "INFO ok=42"
    /// let result: Result<u32, u32> = Err(42).tap_ok_log(Level::INFO, "");     // logs nothing
    /// ```
    fn tap_ok_log<S>(self, level: tracing::Level, ctx: S) -> Self
    where
        Self::Success: std::fmt::Debug,
        S: AsRef<str>;

    /// Logs the [Debug] value of the [Err] variant of [Result] (if any) at the specified
    /// `tracing::Level` with a context message (`ctx`). If `ctx` is empty, then it is omitted.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::log::{TapResultLog, Level};
    ///
    /// let result: Result<u32, u32> = Err(42).tap_err_log(Level::INFO, "hello"); // logs "INFO ctx=hello err=42"
    /// let result: Result<u32, u32> = Err(42).tap_err_log(Level::INFO, "");      // logs "INFO err=42"
    /// let result: Result<u32, u32> = Ok(42).tap_err_log(Level::INFO, "");       // logs nothing
    /// ```
    fn tap_err_log<S>(self, level: tracing::Level, ctx: S) -> Self
    where
        Self::Error: std::fmt::Debug,
        S: AsRef<str>;

    /// Logs the [Debug] value of the [Ok] or [Err] variant of [Result] (if any) at the specified
    /// `tracing::Level` with a context message (`ctx`). If `ctx` is empty, then it is omitted.
    /// Returns self unchanged.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::log::{TapResultLog, Level};
    ///
    /// let result: Result<u32, u32> = Ok(42).tap_result_log(Level::INFO, Level::ERROR, "hello"); // logs "INFO ctx=hello ok=42"
    /// let result: Result<u32, u32> = Ok(42).tap_result_log(Level::INFO, Level::ERROR, "");      // logs "INFO ok=42"
    /// let result: Result<u32, u32> = Err(42).tap_result_log(Level::INFO, Level::ERROR, "");     // logs "ERROR err=42"
    /// ```
    fn tap_result_log<S>(self, ok_level: Level, err_level: Level, ctx: S) -> Self
    where
        Self::Error: std::fmt::Debug,
        Self::Success: std::fmt::Debug,
        S: AsRef<str>;
}

impl<T, E> TapResultLog for Result<T, E> {
    type Error = E;
    type Success = T;

    fn tap_ok_log<S>(self, level: tracing::Level, ctx: S) -> Self
    where
        T: std::fmt::Debug,
        S: AsRef<str>,
    {
        if let Ok(ref ok) = self {
            match (ctx.as_ref(), level) {
                ("", Level::ERROR) => tracing::error!(?ok),
                ("", Level::WARN) => tracing::warn!(?ok),
                ("", Level::INFO) => tracing::info!(?ok),
                ("", Level::DEBUG) => tracing::debug!(?ok),
                ("", Level::TRACE) => tracing::trace!(?ok),

                (ctx, Level::ERROR) => tracing::error!(%ctx, ?ok),
                (ctx, Level::WARN) => tracing::warn!(%ctx, ?ok),
                (ctx, Level::INFO) => tracing::info!(%ctx, ?ok),
                (ctx, Level::DEBUG) => tracing::debug!(%ctx, ?ok),
                (ctx, Level::TRACE) => tracing::trace!(%ctx, ?ok),
            }
        };
        self
    }

    fn tap_err_log<S>(self, level: tracing::Level, ctx: S) -> Self
    where
        E: std::fmt::Debug,
        S: AsRef<str>,
    {
        if let Err(ref err) = self {
            match (ctx.as_ref(), level) {
                ("", Level::ERROR) => tracing::error!(?err),
                ("", Level::WARN) => tracing::warn!(?err),
                ("", Level::INFO) => tracing::info!(?err),
                ("", Level::DEBUG) => tracing::debug!(?err),
                ("", Level::TRACE) => tracing::trace!(?err),

                (ctx, Level::ERROR) => tracing::error!(%ctx, ?err),
                (ctx, Level::WARN) => tracing::warn!(%ctx, ?err),
                (ctx, Level::INFO) => tracing::info!(%ctx, ?err),
                (ctx, Level::DEBUG) => tracing::debug!(%ctx, ?err),
                (ctx, Level::TRACE) => tracing::trace!(%ctx, ?err),
            }
        };
        self
    }

    fn tap_result_log<S>(self, ok_level: Level, err_level: Level, ctx: S) -> Self
    where
        E: std::fmt::Debug,
        T: std::fmt::Debug,
        S: AsRef<str>,
    {
        match self {
            Ok(_) => self.tap_ok_log(ok_level, ctx),
            Err(_) => self.tap_err_log(err_level, ctx),
        }
    }
}
