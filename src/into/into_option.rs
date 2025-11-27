/// Provides postfix conversion of any value into an [Option].
pub trait IntoOption {
    /// Moves a value into a [Some] variant of [Option].
    ///
    /// # Example
    /// ```rust
    /// use fluent_result::into::IntoOption;
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
    /// use fluent_result::into::IntoOption;
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

impl<T> IntoOption for T {}
