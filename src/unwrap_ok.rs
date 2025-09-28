use crate::into::IntoOk;

/// Extension trait for transforming `Result<T, EIn>`` into `Result<T, EOut>`
/// by unwrapping the inner value and rewrapping it in a new error context.
///
/// This trait is useful when the original error type (`EIn`) is should not be possible,
/// or represents a fatal/logic error, and you want to propagate success while substituting
/// a new error type (`EOut`).
///
/// # Type Parameters
/// * `T` - The success type.
pub trait UnwrapOk<T>: crate::internal::Sealed {
    /// Converts `Result<T, EIn>` into `Result<T, EOut>` by unwrapping the value.
    ///
    /// When the destination type is known, `E` can be inferred, otherwise, it can be specified.
    ///
    /// # Panics
    /// Panics if the original result is `Err`, using `unwrap()` internally.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::UnwrapOk;
    /// let result: Result<i32, &str> = Ok(42);
    /// let ok: Result<i32, ()> = result.unwrap_ok();
    /// assert!(ok.is_ok());
    ///
    /// let ok = result.unwrap_ok::<()>();
    /// assert_eq!(ok, Ok(42));
    #[track_caller]
    fn unwrap_ok<EOut>(self) -> Result<T, EOut>;

    /// Converts `Result<T, EIn>` into `Result<T, EOut>` by unwrapping the value with a custom panic message.
    ///
    /// When the destination type is known, `EOut` can be inferred, otherwise, it can be specified.
    ///
    /// # Panics
    /// Panics with the provided message if the original result is `Err`.
    ///
    /// # Example
    /// ```rust
    /// # use result_utils::UnwrapOk;
    /// let result: Result<i32, &str> = Ok(42);
    ///
    /// let ok: Result<i32, ()> = result.expect_ok("Expected Ok");
    /// assert!(ok.is_ok());
    ///
    /// let ok = result.expect_ok::<()>("Expected Ok");
    /// assert_eq!(ok, Ok(42));
    #[track_caller]
    fn expect_ok<EOut>(self, msg: &str) -> Result<T, EOut>;
}

impl<T, EIn: std::fmt::Debug> UnwrapOk<T> for Result<T, EIn> {
    fn unwrap_ok<EOut>(self) -> Result<T, EOut> {
        self.unwrap().into_ok()
    }

    fn expect_ok<EOut>(self, msg: &str) -> Result<T, EOut> {
        self.expect(msg).into_ok()
    }
}
