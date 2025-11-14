use crate::internal;

/// An extension trait for [`bool`] that transforms `true` into [`None`].
pub trait ThenErr: internal::Sealed {
    /// Returns [`Err`] if the [`bool`] value is `true`, and [`Ok(())`](Ok) otherwise.
    ///
    /// This is useful for allowing an early return from a method that returns [`Result`] via
    /// the `?` operator. Or for concicely transforming a boolean guard into a [`Result`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use fluent_result::ThenErr;
    ///
    /// let result = true.then_err("error");
    /// assert_eq!(result, Err("error"));
    ///
    /// let result = false.then_err("error");
    /// assert_eq!(result, Ok(()));
    /// ```
    fn then_err<E>(self, err: E) -> Result<(), E>;
}

/// The implementation of [`ThenErr`] for [`bool`].
impl ThenErr for bool {
    fn then_err<E>(self, err: E) -> Result<(), E> {
        if self { Err(err) } else { Ok(()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_then_err() {
        assert_eq!(true.then_err(42), Err(42));
        assert_eq!(false.then_err(42), Ok(()));
    }
}
