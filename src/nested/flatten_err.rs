use core::error::Error;
use core::fmt::{Display, Formatter};

use derive_more::{IsVariant, TryUnwrap, Unwrap};

/// Allows flattening a [`Result<Result<T, EIn>, EOut>`] into a [`Result<T, NestedError<EIn, EOut>>`].
#[sealed::sealed]
pub trait FlattenErr<T, EIn, EOut>: Sized {
    /// Flattens a [`Result<Result<T, EIn>, EOut>`] into a [`Result<T, NestedError<EIn, EOut>>`].
    ///
    /// # Errors
    ///
    /// - Returns a [`NestedError::Inner`] if the inner [`Result`] is [`Err`].
    /// - Returns a [`NestedError::Outer`] if the outer [`Result`] is [`Err`].
    /// - Returns [`Ok`] if the inner [`Result`] is [`Ok`] and the outer [`Result`] is [`Ok`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fluent_result::nested::{FlattenErr, NestedError};
    ///
    /// let result: Result<Result<i32, &str>, i32> = Ok(Ok(1));
    /// let ok = result.flatten_err().expect("should be ok");
    /// assert_eq!(ok, 1);
    ///
    /// let result: Result<Result<i32, &str>, i32> = Ok(Err("oops"));
    /// let err = result.flatten_err().expect_err("should be err");
    /// assert_eq!(err, NestedError::Inner("oops"));
    ///
    /// let result: Result<Result<i32, &str>, i32> = Err(2);
    /// let err = result.flatten_err().expect_err("should be err");
    /// assert_eq!(err, NestedError::Outer(2));
    /// ```
    fn flatten_err(self) -> Result<T, NestedError<EIn, EOut>>;
}

#[sealed::sealed]
impl<T, EIn, EOut> FlattenErr<T, EIn, EOut> for Result<Result<T, EIn>, EOut> {
    #[inline]
    fn flatten_err(self) -> Result<T, NestedError<EIn, EOut>> {
        match self {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(e)) => Err(NestedError::Inner(e)),
            Err(e) => Err(NestedError::Outer(e)),
        }
    }
}

/// An error created by [`FlattenErr::flatten_err`] a [`Result<Result<T, EIn>, EOut>`].
///
/// # Type Parameters
///
/// - `EIn`: The error type of the inner error.
/// - `EOut`: The error type of the outer container.
#[derive(Debug, PartialEq, Eq, TryUnwrap, IsVariant, Unwrap)]
pub enum NestedError<EIn, EOut> {
    /// The inner most error
    Inner(EIn),
    /// The outer most error
    Outer(EOut),
}

impl<EIn: Display, EOut: Display> Display for NestedError<EIn, EOut> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Outer(e) => write!(f, "{e}"),
            Self::Inner(e) => write!(f, "{e}"),
        }
    }
}

impl<EIn: Error + 'static, EOut: Error + 'static> Error for NestedError<EIn, EOut> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Outer(e) => Some(e),
            Self::Inner(e) => Some(e),
        }
    }
}
