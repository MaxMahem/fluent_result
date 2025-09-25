/// A trait for handling [UnitResult<E>] error cases by sinking the error into a side-effecting function.
pub trait SinkErr<E>: crate::internal::Sealed {
    /// If `self` is Err(e)`, sinks the error value into the provided function.
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

impl<E> SinkErr<E> for crate::UnitResult<E> {
    fn sink_err<F>(self, function: F)
    where
        F: FnOnce(E),
    {
        if let Err(e) = self {
            function(e);
        }
    }
}
