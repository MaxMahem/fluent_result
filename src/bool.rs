use crate::internal;

/// An extension trait for [`bool`] that transforms `true` into [`Err`].
pub trait ThenErr: internal::Sealed {
    /// Returns [`Err`] if the [`bool`] value is `true`, and [`Ok(())`](Ok) otherwise.
    ///
    /// This is useful for allowing an early return from a method that returns [`Result`] via
    /// the `?` operator. Or for concicely transforming a boolean guard into a [`Result`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the [`bool`] value is `true`, and [`Ok(())`](Ok) otherwise.
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

/// An extension trait for [`bool`] that transforms `true` into [`None`].
pub trait ThenNone: internal::Sealed {
    /// Returns [`None`] if the [`bool`] value is `true`, and [`Some(())`](Some) otherwise.
    ///
    /// This is useful for allowing an early return from a method that returns [`Option`] via
    /// the `?` operator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fluent_result::ThenNone;
    ///
    /// fn filter_even_numbers(number: u32) -> Option<u32> {
    ///     (number % 2 == 0).then_none()?;
    ///     Some(number)
    /// }
    ///
    /// assert_eq!(filter_even_numbers(42), None);
    /// assert_eq!(filter_even_numbers(43), Some(43));
    /// ```
    fn then_none(self) -> Option<()>;
}

/// The implementation of [`ThenNone`] for [`bool`].
impl ThenNone for bool {
    fn then_none(self) -> Option<()> {
        if self { None } else { Some(()) }
    }
}
