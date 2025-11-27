/// Allows flattening a [`Result<Result<T, EInner>, EOuter>`] into a [`Result<T, NestedError<EInner, EOuter>>`].
#[sealed::sealed]
pub trait FlattenErr<T, EInner, EOuter>: Sized {
    /// Flattens a [`Result<Result<T, EInner>, EOuter>`] into a [`Result<T, NestedError<EInner, EOuter>>`].
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
    fn flatten_err(self) -> Result<T, NestedError<EInner, EOuter>>;
}

#[sealed::sealed]
impl<T, EInner, EOuter> FlattenErr<T, EInner, EOuter> for Result<Result<T, EInner>, EOuter> {
    fn flatten_err(self) -> Result<T, NestedError<EInner, EOuter>> {
        match self {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(e)) => Err(NestedError::Inner(e)),
            Err(e) => Err(NestedError::Outer(e)),
        }
    }
}

/// An error created by [`FlattenErr::flatten_err`] a [`Result<Result<T, EInner>, EOuter>`].
///
/// # Type Parameters
///
/// - `EIn`: The error type of the inner error.
/// - `EOut`: The error type of the outer container.
#[derive(
    Debug, PartialEq, Eq, derive_more::TryUnwrap, derive_more::IsVariant, derive_more::Unwrap,
)]
pub enum NestedError<EInner, EOuter> {
    /// The inner most error
    Inner(EInner),
    /// The outer most error
    Outer(EOuter),
}

impl<EInner, EOuter> std::fmt::Display for NestedError<EInner, EOuter>
where
    EInner: std::fmt::Display,
    EOuter: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NestedError::Outer(e) => write!(f, "{e}"),
            NestedError::Inner(e) => write!(f, "{e}"),
        }
    }
}

impl<EInner, EOuter> std::error::Error for NestedError<EInner, EOuter>
where
    EInner: std::error::Error + 'static,
    EOuter: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NestedError::Outer(e) => Some(e),
            NestedError::Inner(e) => Some(e),
        }
    }
}
