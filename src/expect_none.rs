#[cfg(doc)]
use std::collections::HashMap;

use crate::internal;

/// An extension trait for [`Option<T>`] that allows unwrapping the [`None`] variant.
pub trait ExpectNone: internal::Sealed {
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
    /// use fluent_result::ExpectNone;
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
    /// use fluent_result::ExpectNone;
    ///
    /// let none: Option<u8> = None;
    /// none.expect_none("test");
    /// ```
    fn expect_none(self, msg: &str);
}

/// Implementation for all [`Option<T>`].
impl<T> ExpectNone for Option<T> {
    fn unwrap_none(self) {
        if self.is_some() {
            panic!("called `Option::unwrap_none()` on a `Some` value");
        }
    }

    fn expect_none(self, msg: &str) {
        if self.is_some() {
            panic!("{}", msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn unwrap_none_panics_on_some() {
        Some(()).unwrap_none();
    }

    #[test]
    fn unwrap_none_unwraps_none_on_none() {
        assert_eq!(None::<u8>.unwrap_none(), ());
    }

    #[test]
    #[should_panic]
    fn expect_none_panics_on_some() {
        Some(()).expect_none("test");
    }

    #[test]
    fn expect_none_unwraps_none_on_none() {
        assert_eq!(None::<u8>.expect_none("test"), ());
    }
}
