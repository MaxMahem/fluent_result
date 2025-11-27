use crate::internal::Sealed;

/// An extension for handling [`Some`] variants by sinking them into a side-effecting function.
pub trait SinkOption<T>: Sealed {
    /// Handles [`Some`] variants of [`Option`]s by sinking them into `sink`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fluent_result::SinkOption;
    ///
    /// let mut log = String::new();
    /// Some("oops").sink(|e| log.push_str(e));
    /// assert_eq!(log, "oops");
    /// ```
    fn sink<F>(self, sink: F)
    where
        F: FnOnce(T);
}

impl<T> SinkOption<T> for Option<T> {
    fn sink<F>(self, sink: F)
    where
        F: FnOnce(T),
    {
        if let Some(e) = self {
            sink(e);
        }
    }
}
