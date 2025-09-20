/// A trait that wraps a value in a `Result::Ok`, allowing ergonomic conversion
/// of values into `Result` types.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
///
/// # Examples
/// ```rust
/// use ok_or::IntoErr;
///
/// let result_owned: Result<(), u32> = Err(42);
/// let result_borrowed: Result<(), &u32> = Err(&42);
/// assert_eq!(result_owned, 42.into_err());
/// assert_eq!(result_borrowed, 42.as_err());
/// ```
pub trait IntoErr<T>: Sized {
    fn into_err(self) -> Result<T, Self>;
    fn as_err(&self) -> Result<T, &Self>;
}

impl<T, E> IntoErr<T> for E {
    fn into_err(self) -> Result<T, Self> {
        Err(self)
    }

    fn as_err(&self) -> Result<T, &Self> {
        Err(&self)
    }
}
