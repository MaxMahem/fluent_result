/// Debug-only assertions for [`Option`] values.
///
/// This module provides the [`ExpectNone`](dbg::ExpectNone) trait that only panics in debug builds.
/// In release builds, the assertions are no-ops.
pub mod dbg {
    #[cfg(doc)]
    use std::collections::HashMap;

    /// An extension trait for [`Option<T>`] that allows unwrapping the [`None`] variant.
    ///
    /// This trait only panics in debug mode (`cfg(debug_assertions)`). In release mode, it does
    /// nothing.
    #[sealed::sealed]
    pub trait ExpectNone {
        /// Unwrap a [`None`] [Option] value, otherwise panic.
        ///
        /// This is useful for validating that a method that should return [`None`] does so. For
        /// example when inserting a value into a [`HashMap`] who's key should be unique.
        ///
        /// # Panics
        /// Only panics in debug builds if the value is a [`Some`] variant.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use fluent_result::expect::expect_none::dbg::ExpectNone;
        ///
        /// let none: Option<u8> = None;
        /// none.unwrap_none();
        /// ```
        fn unwrap_none(self);

        /// Unwrap a [`None`] [Option] value, otherwise panic with `msg`.
        ///
        /// This is useful for validating that a method that should return [`None`] does so. For
        /// example when inserting a value into a [`HashMap`] who's key should be unique.
        ///
        /// # Panics
        ///
        /// Only panics in debug builds with 'msg' if the value is a [`Some`] variant.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use fluent_result::expect::expect_none::dbg::ExpectNone;
        ///
        /// let none: Option<u8> = None;
        /// none.expect_none("test");
        /// ```
        fn expect_none(self, msg: &str);
    }

    #[sealed::sealed]
    impl<T> ExpectNone for Option<T> {
        #[inline]
        #[track_caller]
        fn unwrap_none(self) {
            debug_assert!(
                self.is_none(),
                "called `Option::unwrap_none()` on a `Some` value"
            );
        }

        #[inline]
        #[track_caller]
        fn expect_none(self, msg: &str) {
            debug_assert!(self.is_none(), "{}", msg);
        }
    }
}

/// Release-mode [`assert!`] for [`Option`] values.
///
/// This module provides the [`ExpectNone`](rls::ExpectNone) trait that always panics when
/// assertions fail, regardless of build mode.
pub mod rls {
    #[cfg(doc)]
    use std::collections::HashMap;

    /// An extension trait for [`Option<T>`] that allows unwrapping the [`None`] variant.
    ///
    /// This trait always panics when assertions fail, regardless of build mode.
    #[sealed::sealed]
    pub trait ExpectNone {
        /// Unwrap a [`None`] [`Option`] value, otherwise panic.
        ///
        /// This is useful for validating that a method that should return [`None`] does so. For
        /// example when inserting a value into a [`HashMap`] who's key should be unique.
        ///
        /// # Panics
        /// Always panics if the value is a [`Some`] variant.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use fluent_result::expect::expect_none::rls::ExpectNone;
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
        /// Always panics with 'msg' if the value is a [`Some`] variant.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use fluent_result::expect::expect_none::rls::ExpectNone;
        ///
        /// let none: Option<u8> = None;
        /// none.expect_none("test");
        /// ```
        fn expect_none(self, msg: &str);
    }

    #[sealed::sealed]
    impl<T> ExpectNone for Option<T> {
        #[inline]
        #[track_caller]
        fn unwrap_none(self) {
            assert!(
                self.is_none(),
                "called `Option::unwrap_none()` on a `Some` value"
            );
        }

        #[inline]
        #[track_caller]
        fn expect_none(self, msg: &str) {
            assert!(self.is_none(), "{}", msg);
        }
    }
}
