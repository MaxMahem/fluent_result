/// Provides postfix helpers for transforming an `Option<T>` into `Option<U>`.
pub trait OptionMapTo: crate::internal::Sealed {
    /// Transforms an `Option<T>` into `Option<U>`.
    ///
    /// # Type Parameters
    /// - `U`: The type to transform to.
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::OptionMapTo;
    ///
    /// let some = Some(42).map_to("something");
    /// assert_eq!(some, Some("something"));
    /// ```
    fn map_to<U>(self, value: U) -> Option<U>;
}

impl<T> OptionMapTo for Option<T> {
    fn map_to<U>(self, value: U) -> Option<U> {
        self.map(|_| value)
    }
}
