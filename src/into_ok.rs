/// A trait that wraps a value in a `Result::Ok`, allowing ergonomic conversion
/// of values into `Result` types.
///
/// # Type Parameters
/// - `E`: The error type to use in the `Result`.
///
/// # Examples
/// ```rust
/// use result_utils::IntoOk;
///
/// let result_owned: Result<u32, ()> = Ok(42);
/// let result_borrowed: Result<&u32, ()> = Ok(&42);
/// let result_mut_borrowed: Result<&mut u32, ()> = Ok(&mut 42);
/// assert_eq!(result_owned, 42.into_ok());
/// assert_eq!(result_borrowed, 42.as_ok());
/// assert_eq!(result_mut_borrowed, 42.as_ok_mut());
/// ```
pub trait IntoOk<E> {
    /// Moves the value into a `Ok(self)`.
    fn into_ok(self) -> Result<Self, E>
    where
        Self: Sized,
    {
        Ok(self)
    }

    /// Wraps a borrowed value in a `Ok`.
    fn as_ok(&self) -> Result<&Self, E> {
        Ok(self)
    }

    /// Wraps a mutable borrowed value in a `Ok`.
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
