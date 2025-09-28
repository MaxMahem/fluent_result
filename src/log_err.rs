/// A trait for logging errors from [UnitResult]s (`Result<(), E>`) to a tracing sink.
pub trait LogErr: crate::internal::Sealed {
    /// The error type.
    type Error;

    /// Log the error to the tracing sink as `Level::ERROR`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::LogErr;
    /// Err("oops").log_err(); // Logs at ERROR level
    /// ```
    fn log_err(self)
    where
        Self::Error: std::fmt::Display;

    /// Log the error to the tracing sink at a specific level.
    ///
    /// # Arguments
    /// - `level`: The level to log at.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::LogErr;
    /// use tracing::Level;
    ///
    /// Err("warn").log_err_as(Level::WARN);
    /// ```
    fn log_err_as(self, level: tracing::Level)
    where
        Self::Error: std::fmt::Display;

    /// Log the error to the tracing sink as `Level::ERROR` via a custom formatter.
    ///
    /// # Arguments
    /// - `to_display`: A function that takes the error and returns a `Display` value.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::LogErr;
    /// use std::fmt::Display;
    ///
    /// Err(42).log_err_with(|e| format!("{:#x}", e));
    /// ```
    fn log_err_with<F, D>(self, to_display: F)
    where
        F: FnOnce(Self::Error) -> D,
        D: std::fmt::Display;

    /// Log the error to the tracing sink at a specific level via a custom formatter.
    ///
    /// # Arguments
    /// - `level`: The level to log at.
    /// - `to_display`: A function that takes the error and returns a `Display` value.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::LogErr;
    /// use std::fmt::Display;
    /// use tracing::Level;
    ///
    /// Err(42).log_err_as_with(Level::WARN, |e| format!("{:#x}", e));
    /// ```
    fn log_err_as_with<F, D>(self, level: tracing::Level, to_display: F)
    where
        F: FnOnce(Self::Error) -> D,
        D: std::fmt::Display;

    /// Log the error to the tracing sink as `Level::ERROR` with a custom message.
    ///
    /// # Arguments
    /// - `message`: The message to log.
    ///
    /// # Examples
    /// ```rust
    /// # use result_utils::LogErr;
    /// Err("oops").log_err_msg("Something went wrong");
    /// ```
    fn log_err_msg(self, message: &str);

    /// Log the error to the tracing sink at a specific level with a custom message.
    ///
    /// # Arguments
    /// - `level`: The level to log at.
    /// - `message`: The message to log.
    ///
    /// # Examples
    /// ```rust
    /// # use result_utils::LogErr;
    /// use tracing::Level;
    ///
    /// Err("oops").log_err_msg_as(Level::WARN, "Something went wrong");
    /// ```
    fn log_err_msg_as(self, level: tracing::Level, message: &str);
}

/// Implementation for all [UnitResult<E>].
impl<E> LogErr for crate::UnitResult<E> {
    type Error = E;

    #[track_caller]
    fn log_err(self)
    where
        E: std::fmt::Display,
    {
        match self {
            Err(e) => tracing::error!(error = %e),
            _ => (),
        }
    }

    #[track_caller]
    fn log_err_as(self, level: tracing::Level)
    where
        E: std::fmt::Display,
    {
        match (self, level) {
            (Err(e), tracing::Level::ERROR) => tracing::error!(error = %e),
            (Err(e), tracing::Level::WARN) => tracing::warn!(warn = %e),
            (Err(e), tracing::Level::INFO) => tracing::info!(info = %e),
            (Err(e), tracing::Level::DEBUG) => tracing::debug!(debug = %e),
            (Err(e), tracing::Level::TRACE) => tracing::trace!(trace = %e),
            _ => (),
        }
    }

    #[track_caller]
    fn log_err_with<F, D>(self, to_display: F)
    where
        F: FnOnce(Self::Error) -> D,
        D: std::fmt::Display,
    {
        match self {
            Err(e) => tracing::error!(error = %to_display(e)),
            _ => (),
        }
    }

    #[track_caller]
    fn log_err_as_with<F, D>(self, level: tracing::Level, to_display: F)
    where
        F: FnOnce(Self::Error) -> D,
        D: std::fmt::Display,
    {
        match (self, level) {
            (Err(e), tracing::Level::ERROR) => tracing::error!(error = %to_display(e)),
            (Err(e), tracing::Level::WARN) => tracing::warn!(warn = %to_display(e)),
            (Err(e), tracing::Level::INFO) => tracing::info!(info = %to_display(e)),
            (Err(e), tracing::Level::DEBUG) => tracing::debug!(debug = %to_display(e)),
            (Err(e), tracing::Level::TRACE) => tracing::trace!(trace = %to_display(e)),
            _ => (),
        }
    }

    #[track_caller]
    fn log_err_msg(self, message: &str) {
        match self {
            Err(_) => tracing::error!(%message),
            _ => (),
        }
    }

    #[track_caller]
    fn log_err_msg_as(self, level: tracing::Level, message: &str) {
        match (self, level) {
            (Err(_), tracing::Level::ERROR) => tracing::error!(%message),
            (Err(_), tracing::Level::WARN) => tracing::warn!(%message),
            (Err(_), tracing::Level::INFO) => tracing::info!(%message),
            (Err(_), tracing::Level::DEBUG) => tracing::debug!(%message),
            (Err(_), tracing::Level::TRACE) => tracing::trace!(%message),
            _ => (),
        }
    }
}
