use crate::internal::Sealed;

/// A trait for handling `Result<(), E>` error cases by sinking the error into a side-effecting function.
///
/// This is useful for handling the error case of a unit function.
pub trait SinkErr<E>: Sealed {
    /// If `self` is Err(e)`, calls the provided function with `e`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use result_utils::SinkErr;
    /// let mut captured = None;
    /// Err("oops").sink_err(|e| captured = Some(e));
    /// assert_eq!(captured, Some("oops"));
    /// ```
    fn sink_err<F>(self, function: F)
    where
        F: FnOnce(E);
}

impl<E> SinkErr<E> for Result<(), E> {
    fn sink_err<F>(self, function: F)
    where
        F: FnOnce(E),
    {
        if let Err(e) = self {
            function(e);
        }
    }
}
