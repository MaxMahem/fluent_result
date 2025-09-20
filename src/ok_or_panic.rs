/// Extension trait for transforming [Result<T, EIn>] into `Result<T, EOut>`
/// by unwrapping the inner value and rewrapping it in a new error context.
///
/// This trait is useful when the original error type (`EIn`) is already handled,
/// and you want to propagate success while substituting a new error type (`EOut`).
///
/// # Type Parameters
///
/// * `T` - The success type.
/// * `EOut` - The substituted error type for the outer `Result`.
///
/// # Examples
///
/// ```rust
/// use result_utils::OkOrPanic;
///
/// let res: Result<i32, &str> = Ok(42);
/// let val1: Result<i32, ()> = res.ok_or_unwrap();
/// let val2: Result<i32, ()> = res.ok_or_expect("value missing");
/// assert_eq!(val1, Ok(42));
/// assert_eq!(val2, Ok(42));
pub trait OkOrPanic<T, EOut> {
    /// Converts `Result<T, EIn>` into `Result<T, EOut>` by unwrapping the value.
    ///
    /// # Panics
    ///
    /// Panics if the original result is `Err`, using `unwrap()` internally.
    fn ok_or_unwrap(self) -> Result<T, EOut>;

    /// Converts `Result<T, EIn>` into `Result<T, EOut>` by unwrapping the value,
    /// with a custom panic message if the original result is `Err`.
    ///
    /// # Arguments
    ///
    /// * `msg` - A message to display if the original result is `Err`.
    ///
    /// # Panics
    ///
    /// Panics with the provided message if the original result is `Err`.
    fn ok_or_expect(self, msg: &str) -> Result<T, EOut>;
}

impl<T, EIn: std::fmt::Debug, EOut> OkOrPanic<T, EOut> for Result<T, EIn> {
    fn ok_or_unwrap(self) -> Result<T, EOut> {
        Ok(self.unwrap())
    }

    fn ok_or_expect(self, msg: &str) -> Result<T, EOut> {
        Ok(self.expect(msg))
    }
}
