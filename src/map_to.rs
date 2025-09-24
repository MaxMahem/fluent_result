use crate::internal;

/// Extension trait for transforming `Result<T, E>` into `Result<U, E>`
pub trait MapTo: internal::Sealed {
    type Error;

    /// Maps `Result<T, E>` into `Result<U, E>` by returning `Ok(other)` if the value is [Ok],
    /// leaving an [Err] variant unchanged.
    ///
    /// Analogous to [Result::map(|_| other)](Result::map), but uses a fixed value instead of calling a function.
    ///
    /// # Type Parameters
    /// - `TOther`: The type of the value to return in the `Ok` variant.    
    ///
    /// # Example
    ///```rust
    /// # use result_utils::MapTo;
    /// let result: Result<bool, ()> = Ok(42).map_to(true);
    /// ```
    fn map_to<TOther>(self, other: TOther) -> Result<TOther, Self::Error>;
}

/// Implementation for all `Result<T, E>`.
impl<T, E> MapTo for Result<T, E> {
    type Error = E;

    fn map_to<TOther>(self, other: TOther) -> Result<TOther, Self::Error> {
        match self {
            Ok(_) => Ok(other),
            Err(e) => Err(e),
        }
    }
}
