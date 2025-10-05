use crate::log::Level;

/// Provides a postfix method to passthrough and log the [Debug] value of [Option] variants with [tracing].
pub trait TapOptionLog: crate::internal::Sealed {
    /// The type of the [Some] value.
    type Some;

    /// Logs the [Debug] value of the [Some] variant of [Option] (if any) at the specified `tracing::Level`
    /// with a context message (`ctx`). If `ctx` is empty, then it is omitted. Returns self unchanged.
    ///
    /// This method is useful for logging values in method chains or pipelines without disrupting flow.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::log::{TapOptionLog, Level};
    ///
    /// let option: Option<u32> = Some(42).tap_some_log(Level::INFO, "hello");  // logs "INFO ctx=hello ok=42"
    /// let option: Option<u32> = Some(42).tap_some_log(Level::INFO, "");       // logs "INFO ok=42"
    /// let option: Option<u32> = None.tap_some_log(Level::INFO, "");           // logs nothing
    /// ```
    fn tap_some_log<S>(self, level: tracing::Level, ctx: S) -> Self
    where
        Self::Some: std::fmt::Debug,
        S: AsRef<str>;

    /// Logs a message if the [Option] is [None] at the specified `tracing::Level`.
    ///
    /// This method is useful for logging values in method chains or pipelines without disrupting flow.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::log::{TapOptionLog, Level};
    ///
    /// let option: Option<u32> = None.tap_none_log(Level::INFO, "hello");   // logs "INFO msg=hello"
    /// let option: Option<u32> = Some(42).tap_none_log(Level::INFO, "");     // logs nothing
    /// ```
    fn tap_none_log<S: AsRef<str>>(self, level: tracing::Level, msg: S) -> Self;
}

impl<T> TapOptionLog for Option<T> {
    type Some = T;

    fn tap_some_log<S>(self, level: tracing::Level, ctx: S) -> Self
    where
        T: std::fmt::Debug,
        S: AsRef<str>,
    {
        if let Some(ref some) = self {
            match (ctx.as_ref(), level) {
                ("", Level::ERROR) => tracing::error!(?some),
                ("", Level::WARN) => tracing::warn!(?some),
                ("", Level::INFO) => tracing::info!(?some),
                ("", Level::DEBUG) => tracing::debug!(?some),
                ("", Level::TRACE) => tracing::trace!(?some),

                (ctx, Level::ERROR) => tracing::error!(%ctx, ?some),
                (ctx, Level::WARN) => tracing::warn!(%ctx, ?some),
                (ctx, Level::INFO) => tracing::info!(%ctx, ?some),
                (ctx, Level::DEBUG) => tracing::debug!(%ctx, ?some),
                (ctx, Level::TRACE) => tracing::trace!(%ctx, ?some),
            }
        };
        self
    }

    fn tap_none_log<S: AsRef<str>>(self, level: tracing::Level, msg: S) -> Self {
        if self.is_none() {
            let msg = msg.as_ref();
            match level {
                Level::ERROR => tracing::error!(%msg),
                Level::WARN => tracing::warn!(%msg),
                Level::INFO => tracing::info!(%msg),
                Level::DEBUG => tracing::debug!(%msg),
                Level::TRACE => tracing::trace!(%msg),
            }
        };
        self
    }
}
