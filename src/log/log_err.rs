use crate::UnitResult;
use crate::log::{Level, TapLog};

/// Provides postfix handlers for [Err] variants of `UnitResult` (Result<(), E>) via logging them with [tracing].
pub trait LogErr: crate::internal::Sealed {
    /// The error type.
    type Error;

    /// Handles [Err] variant by logging the [Debug] representation at `Level::ERROR``.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::UnitResult;
    /// use fluent_result::log::{LogErr, Level};
    ///
    /// let error: UnitResult<&str> = Err("oops");
    /// let valid: UnitResult<&str> = Ok(());
    ///
    /// error.log_err(Level::ERROR, "my bad");    // Logs ERROR: ctx="my bad" err="oops"
    /// error.log_err(Level::ERROR, "");          // Logs ERROR: err="oops"
    /// valid.log_err(Level::ERROR, "anything");  // logs nothing
    /// ```
    fn log_err(self, level: Level, ctx: &str)
    where
        Self::Error: std::fmt::Debug;
}

/// Implementation for all [UnitResult].
impl<E> LogErr for UnitResult<E> {
    type Error = E;

    fn log_err(self, level: tracing::Level, ctx: &str)
    where
        E: std::fmt::Debug,
    {
        _ = self.tap_err_log(level, ctx);
    }
}
