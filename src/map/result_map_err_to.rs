use crate::internal;

/// Extension trait for transforming `Result<T, E1>` into `Result<T, E2>`
pub trait ResultMapErrTo: internal::Sealed {
    /// The success type.
    type Success;

    /// Maps `Result<T, E1>` into `Result<T, E2>` by substituting `Err(other)` for the [Err] variant.
    ///
    /// Analogous to [Result::map_err(|_| other)](Result::map_err), but with a fixed value.
    ///
    /// # Example
    ///```rust
    /// use result_utils::map::ResultMapErrTo;
    ///
    /// let result: Result<(), bool> = Err(42).map_err_to(false);
    /// assert_eq!(result, Err(false));
    /// ```
    fn map_err_to<EOther>(self, other: EOther) -> Result<Self::Success, EOther>;
}

/// Implementation for all `Result<T, E>`.
impl<T, E> ResultMapErrTo for Result<T, E> {
    type Success = T;

    fn map_err_to<EOther>(self, other: EOther) -> Result<Self::Success, EOther> {
        self.map_err(|_| other)
    }
}
