/// Extension trait for transforming `Result<T, EIn>` into `Result<T, EOut>`
/// by unwrapping the inner value and rewrapping it in a new error context.
///
/// This trait is useful when the original error type (`EIn`) is irrelevant
/// or already handled, and you want to propagate success while substituting
/// a new error type (`EOut`)—typically for layering or API consistency.
///
/// # Type Parameters
///
/// * `T` - The success type.
/// * `EOut` - The substituted error type for the outer `Result`.
pub trait OkOr<T, EOut> {
    /// Converts `Result<T, EIn>` into `Result<T, EOut>` by unwrapping the value.
    ///
    /// # Panics
    ///
    /// Panics if the original result is `Err`, using `unwrap()` internally.
    ///
    /// # Examples
    ///
    /// ```
    /// let res: Result<i32, &str> = Ok(42);
    /// let val: Result<i32, ()> = res.ok_or_unwrap();
    /// assert_eq!(val, Ok(42));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// let res: Result<i32, &str> = Ok(7);
    /// let val: Result<i32, ()> = res.ok_or_expect("value missing");
    /// assert_eq!(val, Ok(7));
    /// ```
    fn ok_or_expect(self, msg: &str) -> Result<T, EOut>;
}

impl<T, EIn: std::fmt::Debug, EOut> OkOr<T, EOut> for Result<T, EIn> {
    fn ok_or_unwrap(self) -> Result<T, EOut> {
        Ok(self.unwrap())
    }

    fn ok_or_expect(self, msg: &str) -> Result<T, EOut> {
        Ok(self.expect(msg))
    }
}
