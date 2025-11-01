/// A postfix helper extension for handling [`Some`] variants by sinking them into a side-effecting function.
///
/// # Type Parameters
/// - `T`: The option type.
pub trait Sink<T>: crate::internal::Sealed {
    /// Handles [`Some`] variants of [`Option`]s by sinking them into `sink`.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::HandleSome;
    ///
    /// let mut log = String::new();
    /// Some("oops").sink(|e| log.push_str(e));
    /// assert_eq!(log, "oops");
    /// ```
    fn sink<F>(self, sink: F)
    where
        F: FnOnce(T);
}

impl<T> Sink<T> for Option<T> {
    fn sink<F>(self, sink: F)
    where
        F: FnOnce(T),
    {
        if let Some(e) = self {
            sink(e);
        }
    }
}
