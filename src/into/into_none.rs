/// A trait that converts a value into an `Option::Some`.
pub trait IntoNone {
    /// Turns a value into [None].
    ///
    /// When used in terminal position, `TOut` can be inferred.
    /// When used in other positions, `TOut` can be specified.
    ///
    /// # Type Parameters
    /// - `TOut`: The type of the [None] variant.
    ///
    /// # Example
    /// ```rust
    /// use result_utils::into::IntoNone;
    ///
    /// let some: Option<&str> = 42.into_none();
    /// assert!(some.is_none());
    ///
    /// let some = 42.into_none::<&str>();
    /// assert!(some.is_none());
    /// ```
    fn into_none<TOut>(self) -> Option<TOut>
    where
        Self: Sized,
    {
        None
    }
}

impl<T> IntoNone for T {
    fn into_none<TOut>(self) -> Option<TOut> {
        None
    }
}
