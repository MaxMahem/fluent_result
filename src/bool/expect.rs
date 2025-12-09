/// Debug-only [`assert!`]s for [`bool`].
///
/// This module provides an [`Expect`](dbg::Expect) trait that only panics in debug builds.
/// In release builds, the assertions are no-ops.
pub mod dbg {
    /// An extension trait for [`bool`] that provides [`assert!`] wrapper methods for expected
    /// `true` and `false` values.
    ///
    /// This trait only panics in debug mode (`cfg(debug_assertions)`). In release mode, it does
    /// nothing.
    #[sealed::sealed]
    pub trait Expect {
        /// [`assert!`]s that the value is `true` with `msg` as the assertion message.
        ///
        /// Only panics in debug builds if the value is `false`.
        fn expect_true(self, msg: &str);
        /// [`assert!`]s that the value is `false` with `msg` as the assertion message.
        ///
        /// Only panics in debug builds if the value is `true`.
        fn expect_false(self, msg: &str);
    }

    #[sealed::sealed]
    impl Expect for bool {
        #[inline]
        #[track_caller]
        fn expect_true(self, msg: &str) {
            debug_assert!(self, "{}", msg);
        }

        #[inline]
        #[track_caller]
        fn expect_false(self, msg: &str) {
            debug_assert!(!self, "{}", msg);
        }
    }
}

/// Release-mode [`assert!`]s for [`bool`] values.
///
/// This module provides the [`Expect`](rls::Expect) trait that always panics when assertions fail,
/// regardless of build mode.
pub mod rls {
    /// An extension trait for [`bool`] that provides [`assert!`] wrapper methods for expected
    /// `true` and `false` values.
    ///
    /// This trait always panics when assertions fail, regardless of build mode.
    #[sealed::sealed]
    pub trait Expect {
        /// [`assert!`]s that the value is `true` with `msg` as the assertion message.
        ///
        /// Always panics if the value is `false`.
        fn expect_true(self, msg: &str);

        /// [`assert!`]s that the value is `false` with `msg` as the assertion message.
        ///
        /// Always panics if the value is `true`.
        fn expect_false(self, msg: &str);
    }

    #[sealed::sealed]
    impl Expect for bool {
        #[inline]
        #[track_caller]
        fn expect_true(self, msg: &str) {
            assert!(self, "{}", msg);
        }

        #[inline]
        #[track_caller]
        fn expect_false(self, msg: &str) {
            assert!(!self, "{}", msg);
        }
    }
}
