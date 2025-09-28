use crate::internal::Sealed;
use crate::into_ok::IntoOk;

/// Extension trait for transforming `Result<T, EIn>`` into `Result<T, EOut>`
/// by unwrapping the inner value and rewrapping it in a new error context.
///
/// This trait is useful when the original error type (`EIn`) is should not be possible
/// and you want to propagate success while substituting a new error type (`EOut`).
///
/// # Type Parameters
/// * `T` - The success type.
pub trait IntoOkOrPanic<T>: Sealed {
    /// Converts `Result<T, EIn>` into `Result<T, EOut>` by unwrapping the value.
    ///
    /// When the destination type is known, `E` can be inferred, otherwise, `E` can be specified.
    ///
    /// # Type Parameters
    /// * `EOut` - The error type in the Result after conversion.
    ///
    /// # Panics
    /// Panics if the original result is `Err`, using `unwrap()` internally.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoOkOrPanic;
    /// let result: Result<i32, &str> = Ok(42);
    /// let ok: Result<i32, ()> = result.into_ok_or_unwrap();
    /// assert!(ok.is_ok());
    ///
    /// let ok = result.into_ok_or_unwrap::<()>();
    /// assert_eq!(ok, Ok(42));
    #[track_caller]
    fn into_ok_or_unwrap<EOut>(self) -> Result<T, EOut>;

    /// Converts `Result<T, EIn>` into `Result<T, EOut>` by unwrapping the value,
    /// with a custom panic message if the original result is `Err`.
    ///
    /// When the destination type is known, `E` can be inferred, otherwise, `E` can be specified.
    ///
    /// # Type Parameters
    /// * `EOut` - The error type in the Result after conversion.
    ///
    /// # Arguments
    /// * `msg` - A message to display if the original result is `Err`.
    ///
    /// # Panics
    /// Panics with the provided message if the original result is `Err`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::IntoOkOrPanic;
    /// let result: Result<i32, &str> = Ok(42);
    ///
    /// let ok: Result<i32, ()> = result.into_ok_or_expect("Expected Ok");
    /// assert!(ok.is_ok());
    ///
    /// let ok = result.into_ok_or_expect::<()>("Expected Ok");
    /// assert_eq!(ok, Ok(42));
    #[track_caller]
    fn into_ok_or_expect<EOut>(self, msg: &str) -> Result<T, EOut>;
}

impl<T, EIn: std::fmt::Debug> IntoOkOrPanic<T> for Result<T, EIn> {
    fn into_ok_or_unwrap<EOut>(self) -> Result<T, EOut> {
        self.unwrap().into_ok()
    }

    fn into_ok_or_expect<EOut>(self, msg: &str) -> Result<T, EOut> {
        self.expect(msg).into_ok()
    }
}
