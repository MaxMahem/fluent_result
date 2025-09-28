/// A trait for transforming an `Option<T>` into `Option<U>`.
pub trait OptionMapTo<T> {
    /// Transforms an `Option<T>` into `Option<U>`.
    ///
    /// # Example
    /// ```rust
    /// use result_utils::map::OptionMapTo;
    ///
    /// let some = Some(42).map_to("something");
    /// assert_eq!(some, Some("something"));
    /// ```
    fn map_to<U>(self, value: U) -> Option<U>;
}

impl<T> OptionMapTo<T> for Option<T> {
    fn map_to<U>(self, value: U) -> Option<U> {
        self.map(|_| value)
    }
}
