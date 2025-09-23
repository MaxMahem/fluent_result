/// A trait that converts a value into a `Result::Ok`.
pub trait IntoOk {
    /// Moves a value into a `Result::Ok`.
    ///
    /// When used in terminal position, `E` can be inferred.
    /// When used in other positions, `E` can be specified.
    ///
    /// # Type Parameters
    /// - `E`: The error type to use in the `Result`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoOk;
    /// let owned_result: Result<u32, ()> = 42.into_ok();
    /// assert!(owned_result.is_ok());
    /// let owned_result = 42.into_ok::<()>();
    /// assert!(owned_result.is_ok());
    fn into_ok<E>(self) -> Result<Self, E>
    where
        Self: Sized,
    {
        Ok(self)
    }

    /// Wraps a borrowed value in a `Result::Ok`.
    ///
    /// When used in terminal position, `E` can be inferred.
    /// When used in other positions, `E` can be specified.
    ///
    /// # Type Parameters
    /// - `E`: The error type to use in the `Result`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoOk;
    /// let borrowed_result: Result<&u32, ()> = 42.as_ok();
    /// assert!(borrowed_result.is_ok());
    /// let borrowed_result = 42.into_ok::<()>();
    /// assert!(borrowed_result.is_ok());
    fn as_ok<E>(&self) -> Result<&Self, E> {
        Ok(self)
    }

    /// Wraps a mutable borrowed value in a `Ok`.
    ///
    /// When used in terminal position, `E` can be inferred.
    /// When used in other positions, `E` can be specified.
    ///
    /// # Type Parameters
    /// - `E`: The error type to use in the `Result`.
    ///
    /// # Examples
    /// ```rust
    /// # use result_utils::IntoOk;
    /// let borrowed_mut_result: Result<&u32, ()> = 42.as_ok();
    /// assert!(borrowed_mut_result.is_ok());
    /// let borrowed_mut_result = 42.into_ok::<()>();
    /// assert!(borrowed_mut_result.is_ok());
    fn as_ok_mut<E>(&mut self) -> Result<&mut Self, E> {
        Ok(self)
    }
}

/// Implements `IntoOk<T>` for all types.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
impl<T> IntoOk for T {}
