/// A trait for handling [UnitResult<E>] error cases by sinking the error into a side-effecting function.
///
/// # Type Parameters
/// - `E`: The error type.
pub trait HandleErr<E>: crate::internal::Sealed {
    /// Handles [Err] variants of [UnitResult]s by sinking the error into `sink`.
    ///
    /// # Example
    /// ```rust
    /// use result_utils::HandleErr;
    ///
    /// let mut captured = None;
    /// Err("oops").handle_err(|e| captured = Some(e));
    /// assert_eq!(captured, Some("oops"));
    /// ```
    fn handle_err<F>(self, sink: F)
    where
        F: FnOnce(E);
}

impl<E> HandleErr<E> for crate::UnitResult<E> {
    fn handle_err<F>(self, function: F)
    where
        F: FnOnce(E),
    {
        if let Err(e) = self {
            function(e);
        }
    }
}
