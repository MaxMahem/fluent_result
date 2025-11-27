#[cfg(doc)]
use std::collections::HashMap;

/// An extension trait for [`Option<T>`] that allows unwrapping the [`None`] variant.
#[sealed::sealed]
pub trait ExpectNone {
    /// Unwrap a [`None`] [Option] value, otherwise panic.
    ///
    /// This is useful for validating that a method that should return [`None`] does so. For
    /// example when inserting a value into a [`HashMap`] who's key should be unique.
    ///
    /// # Panics
    /// Panics if the value is a [`Some`] variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fluent_result::expect::ExpectNone;
    ///
    /// let none: Option<u8> = None;
    /// none.unwrap_none();
    /// ```
    fn unwrap_none(self);

    /// Unwrap a [`None`] [Option] value, otherwise panic with the provided message.
    ///
    /// This is useful for validating that a method that should return [`None`] does so. For
    /// example when inserting a value into a [`HashMap`] who's key should be unique.
    ///
    /// # Panics
    ///
    /// Panics with 'msg' if the value is a [`Some`] variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fluent_result::expect::ExpectNone;
    ///
    /// let none: Option<u8> = None;
    /// none.expect_none("test");
    /// ```
    fn expect_none(self, msg: &str);
}

/// Implementation for all [`Option<T>`].
#[sealed::sealed]
impl<T> ExpectNone for Option<T> {
    fn unwrap_none(self) {
        assert!(
            self.is_none(),
            "called `Option::unwrap_none()` on a `Some` value"
        );
    }

    fn expect_none(self, msg: &str) {
        assert!(self.is_none(), "{}", msg);
    }
}
