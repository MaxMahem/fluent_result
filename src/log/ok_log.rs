use crate::log::{Level, TapResultLog};

/// Provides postfix helpers for transforming [Result<T, E>] into `Option<T>` by logging the [Err] variant with [tracing].
pub trait OkLog: crate::internal::Sealed {
    /// The type of the success value.
    type Success;

    /// The type of the error value.
    type Error;

    /// Transform [Result<T, E>] into `Option<T>` by logging the [Debug] value of [Err] variants at the specified [tracing::Level]
    /// with a contextual message (`ctx`). If `ctx` is empty, then it is omitted.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::log::{OkLog, Level};
    ///
    /// let none: Option<u32> = Err(42).ok_log(Level::INFO, "hello"); // logs "INFO ctx=hello err=42"
    /// assert!(none.is_none());
    ///
    /// let none: Option<u32> = Err(42).ok_log(Level::INFO, ""); // logs "INFO err=42"
    /// assert!(none.is_none());
    ///
    /// let some = Ok::<u32, &str>(42).ok_log(Level::INFO, "hello"); // logs nothing
    /// assert!(some.is_some());
    /// ```
    fn ok_log<S>(self, level: Level, ctx: S) -> Option<Self::Success>
    where
        Self::Error: std::fmt::Debug,
        S: AsRef<str>;
}

impl<T, E> OkLog for Result<T, E> {
    type Success = T;
    type Error = E;

    fn ok_log<S>(self, level: Level, ctx: S) -> Option<Self::Success>
    where
        E: std::fmt::Debug,
        S: AsRef<str>,
    {
        self.tap_err_log(level, ctx).ok()
    }
}
