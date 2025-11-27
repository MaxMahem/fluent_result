/// An extension for handling [`Some`] variants by sinking them into a side-effecting function.
#[sealed::sealed]
pub trait SinkOption<T> {
    /// Handles [`Some`] variants of [`Option`]s by sinking them into `sink`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fluent_result::sink::SinkOption;
    ///
    /// let mut log = String::new();
    /// Some("oops").sink(|e| log.push_str(e));
    /// assert_eq!(log, "oops");
    /// ```
    fn sink<F>(self, sink: F)
    where
        F: FnOnce(T);
}

#[sealed::sealed]
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
