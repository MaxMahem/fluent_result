/// A trait that wraps a value in a `Result::Ok`, allowing ergonomic conversion
/// of values into `Result` types.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
pub trait IntoErr<T> {
    /// Move a value into a `Result::Err`.
    fn into_err(self) -> Result<T, Self>
    where
        Self: Sized,
    {
        Err(self)
    }

    /// Wraps a borrowed value in a `Result::Err`.
    fn as_err(&self) -> Result<T, &Self> {
        Err(self)
    }
}

/// Implements `IntoErr<T>` for any error type `E` that implements `std::error::Error`.
///
/// # Type Parameters
/// - `T`: The success type to use in the `Result`.
/// - `E`: The error type, which must implement `std::error::Error`.
impl<T, E: std::error::Error> IntoErr<T> for E {}
