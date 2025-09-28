/// A trait that converts a value into an `Option::Some`.
pub trait IntoSome {
    /// Moves a value into an `Option::Some`.
    ///
    /// # Example
    /// ```rust
    /// use result_utils::into::IntoSome;
    ///
    /// let some = 42.into_some();
    /// assert!(some.is_some());
    /// ```
    fn into_some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self)
    }
}

impl<T> IntoSome for T {}
