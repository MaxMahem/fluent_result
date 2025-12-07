#[cfg(doc)]
use crate::nested::FlattenErr;

/// Allows handling nested [`Result`]s by boxing errors into a [`Box<dyn Error>`].
///
/// This trait provides a way to convert nested [`Result`] types with different [`std::error::Error`] error types into
/// a single-level [`Result`] with a boxed error type [`Box<dyn Error>`]. This is useful when working with operations
/// that produce nested [`Result`]s and erasing the error type is acceptable. This trait works with [`Result`]s with up
/// to four layers of nesting, so long as all error types implement [`std::error::Error`].
///
/// If all the error types are the same, consider using [`Result::flatten`] instead. For results with only two layers of
/// nesting, consider using [`FlattenErr::flatten_err`].
///
/// # Type Parameters
///
/// - `T`: The success type of the result.
///
/// # Examples
///
/// ```rust
/// use std::error::Error;
/// use fluent_result::nested::BoxErr;
///
/// let result: Result<Result<i32, std::io::Error>, std::io::Error> = Ok(Ok(42));
/// let boxed: Result<i32, Box<dyn Error>> = result.box_err();
/// assert_eq!(boxed.unwrap(), 42);
///
/// let err_io = std::io::Error::from(std::io::ErrorKind::NotFound);
/// let result: Result<Result<i32, std::io::Error>, std::fmt::Error> = Ok(Err(err_io));
/// let boxed: Result<i32, Box<dyn Error>> = result.box_err();
/// assert!(boxed.is_err());
/// ```
#[sealed::sealed]
pub trait BoxErr<T> {
    /// Boxes the error in a [`Result`], flattening any nesting.
    ///
    /// For a single-level [`Result<T, E>`], this boxes the error. For nested [`Result`]s, this
    /// recursively flattens all levels into a single [`Result<T, Box<dyn Error>>`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if any level of the nested [`Result`] is an error, with the
    /// error boxed as [`Box<dyn Error>`].
    fn box_err(self) -> Result<T, Box<dyn std::error::Error>>;
}

/// Implementation for Result<T, E>
#[sealed::sealed]
impl<T, E> BoxErr<T> for Result<T, E>
where
    E: std::error::Error + 'static,
{
    #[inline]
    fn box_err(self) -> Result<T, Box<dyn std::error::Error>> {
        self.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

/// Double-nested Results: Result<Result<T, E1>, E2>
#[sealed::sealed]
impl<T, E1, E2> BoxErr<T> for Result<Result<T, E1>, E2>
where
    E1: std::error::Error + 'static,
    E2: std::error::Error + 'static,
{
    #[inline]
    fn box_err(self) -> Result<T, Box<dyn std::error::Error>> {
        match self {
            Ok(inner) => inner.box_err(),
            Err(e) => Err(Box::new(e)),
        }
    }
}

/// Triple-nested Results: Result<Result<Result<T, E1>, E2>, E3>
#[sealed::sealed]
impl<T, E1, E2, E3> BoxErr<T> for Result<Result<Result<T, E1>, E2>, E3>
where
    E1: std::error::Error + 'static,
    E2: std::error::Error + 'static,
    E3: std::error::Error + 'static,
{
    #[inline]
    fn box_err(self) -> Result<T, Box<dyn std::error::Error>> {
        match self {
            Ok(inner) => inner.box_err(),
            Err(e) => Err(Box::new(e)),
        }
    }
}

/// Quadruple-nested Results: Result<Result<Result<Result<T, E1>, E2>, E3>, E4>>
#[sealed::sealed]
impl<T, E1, E2, E3, E4> BoxErr<T> for Result<Result<Result<Result<T, E1>, E2>, E3>, E4>
where
    E1: std::error::Error + 'static,
    E2: std::error::Error + 'static,
    E3: std::error::Error + 'static,
    E4: std::error::Error + 'static,
{
    #[inline]
    fn box_err(self) -> Result<T, Box<dyn std::error::Error>> {
        match self {
            Ok(inner) => inner.box_err(),
            Err(e) => Err(Box::new(e)),
        }
    }
}
