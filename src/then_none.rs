use crate::internal;

pub trait ThenNone: internal::Sealed {
    /// Returns [`None`] if the [`bool`] value is `true`, and [`Some(())`](Some) otherwise.
    ///
    /// This is useful for allowing an early return from a method that returns [`Option`] via
    /// the `?` operator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fluent_result::then_none;
    ///
    /// fn filter_even_numbers(number: u32) -> Option<u32> {
    ///     (number % 2 == 0).then_none()?
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
