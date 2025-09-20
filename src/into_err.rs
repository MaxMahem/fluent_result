/// A trait that wraps a value in a `Result::Ok`, allowing ergonomic conversion
/// of values into `Result` types.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
pub trait IntoErr<T>: Sized {
    /// Wraps a value in a `Result::Err`, allowing for ergonomic conversion
    /// of error values into `Result` types.
    fn into_err(self) -> Result<T, Self>;

    /// Wraps a reference to a value in a `Result::Err`, allowing for
    /// low-overhead conversion of error references into `Result` types.
    fn as_err(&self) -> Result<T, &Self>;
}

/// Implements `IntoErr<T>` for any error type `E` that implements `std::error::Error`.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
/// - `E`: The error type, which must implement `std::error::Error`.
impl<T, E: std::error::Error> IntoErr<T> for E {
    fn into_err(self) -> Result<T, Self> {
        Err(self)
    }

    fn as_err(&self) -> Result<T, &Self> {
        Err(&self)
    }
}
