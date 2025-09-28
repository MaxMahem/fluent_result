/// A trait that converts a value into an `Option::Some`.
pub trait IntoSome {
    /// Moves a value into an `Option::Some`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoSome;
    /// let some = 42.into_some();
    /// assert!(some.is_some());
    /// ```
    fn into_some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self)
    }

    /// Wraps a borrowed value in an `Option::Some`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoSome;
    /// let some = 42.as_some();
    /// assert!(some.is_some());
    /// ```
    fn as_some(&self) -> Option<&Self> {
        Some(self)
    }
}

impl<T> IntoSome for T {}
