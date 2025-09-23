/// A trait that converts a value into a `Result::Ok`.
///
/// # Type Parameters
/// - `E`: The error type to use in the `Result`.
pub trait IntoOk<E> {
    /// Moves a value into a `Result::Ok`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoOk;
    /// let ok: Result<u32, ()> = Ok(42);
    /// assert_eq!(ok, 42.into_ok());
    fn into_ok(self) -> Result<Self, E>
    where
        Self: Sized,
    {
        Ok(self)
    }

    /// Wraps a borrowed value in a `Result::Ok`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoOk;
    /// let ok: Result<&u32, ()> = Ok(&42);
    /// assert_eq!(ok, 42.as_ok());
    fn as_ok(&self) -> Result<&Self, E> {
        Ok(self)
    }

    /// Wraps a mutable borrowed value in a `Ok`.
    ///
    /// # Examples
    /// ```rust
    /// # use result_utils::IntoOk;
    /// let ok: Result<&mut u32, ()> = Ok(&mut 42);
    /// assert_eq!(ok, 42.as_ok_mut());
    fn as_ok_mut(&mut self) -> Result<&mut Self, E> {
        Ok(self)
    }
}

/// Implements `IntoOk<T>` for all types.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
/// - `E`: The error type.
impl<T, E> IntoOk<E> for T {}
