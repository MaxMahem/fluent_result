use crate::internal;

/// An extension trait for [`bool`] that provides conversion methods to [`Result`] and [`Option`].
///
/// This trait allows transforming boolean values into [`Result`] or [`Option`] types,
/// making it easy to use boolean conditions with the `?` operator for early returns.
pub trait Then: internal::Sealed {
    /// Returns [`Err`] if the [`bool`] value is `true`, and [`Ok(())`](Ok) otherwise.
    ///
    /// This is useful for allowing an early return from a method that returns [`Result`] via
    /// the `?` operator. Or for concisely transforming a boolean guard into a [`Result`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the [`bool`] value is `true`, and [`Ok(())`](Ok) otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fluent_result::bool::Then;
    ///
    /// let result = true.then_err("error");
    /// assert_eq!(result, Err("error"));
    ///
    /// let result = false.then_err("error");
    /// assert_eq!(result, Ok(()));
    /// ```
    fn then_err<E>(self, err: E) -> Result<(), E>;

    /// Returns [`None`] if the [`bool`] value is `true`, and [`Some(())`](Some) otherwise.
    ///
    /// This is useful for allowing an early return from a method that returns [`Option`] via
    /// the `?` operator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fluent_result::bool::Then;
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

/// The implementation of [`Then`] for [`bool`].
impl Then for bool {
    fn then_err<E>(self, err: E) -> Result<(), E> {
        if self { Err(err) } else { Ok(()) }
    }

    fn then_none(self) -> Option<()> {
        if self { None } else { Some(()) }
    }
}
