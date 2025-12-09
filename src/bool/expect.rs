/// [`debug_assert!`]s for `bool` values.
pub mod dbg {
    /// An extension trait for `bool` that allows [`debug_assert!`]ing `bool` values.
    #[sealed::sealed]
    pub trait Expect {
        /// [`debug_assert!`]s that the value is `true`.
        ///
        /// # Panics
        ///
        /// Panics in debug builds if the value is `false`.
        fn assert_true(self);

        /// [`debug_assert!`]s that the value is `true` with `msg` as the assertion message.
        ///
        /// # Panics
        ///
        /// Panics in debug builds if the value is `false`.
        fn expect_true(self, msg: &str);

        /// [`debug_assert!`]s that the value is `false`.
        ///
        /// # Panics
        ///
        /// Panics in debug builds if the value is `true`.
        fn assert_false(self);

        /// [`debug_assert!`]s that the value is `false` with `msg` as the assertion message.
        ///
        /// # Panics
        ///
        /// Panics in debug builds if the value is `true`.
        fn expect_false(self, msg: &str);
    }

    #[sealed::sealed]
    impl Expect for bool {
        #[inline]
        #[track_caller]
        fn assert_true(self) {
            debug_assert!(self, "assertion failed: expected `true` but was `false`");
        }

        #[inline]
        #[track_caller]
        fn expect_true(self, msg: &str) {
            debug_assert!(self, "{}", msg);
        }

        #[inline]
        #[track_caller]
        fn assert_false(self) {
            debug_assert!(!self, "assertion failed: expected `false` but was `true`");
        }

        #[inline]
        #[track_caller]
        fn expect_false(self, msg: &str) {
            debug_assert!(!self, "{}", msg);
        }
    }
}

/// [`assert!`]s for `bool` values.
pub mod rls {
    /// An extension trait for `bool` that allows [`assert!`]ing `bool` values.
    #[sealed::sealed]
    pub trait Expect {
        /// [`assert!`]s that the value is `true`.
        ///
        /// # Panics
        ///
        /// Panics if the value is `false`.
        fn assert_true(self);

        /// [`assert!`]s that the value is `true` with `msg` as the assertion message.
        ///
        /// # Panics
        ///
        /// Panics if the value is `false`.
        fn expect_true(self, msg: &str);

        /// [`assert!`]s that the value is `false`.
        ///
        /// # Panics
        ///
        /// Panics if the value is `true`.
        fn assert_false(self);

        /// [`assert!`]s that the value is `false` with `msg` as the assertion message.
        ///
        /// # Panics
        ///
        /// Panics if the value is `true`.
        fn expect_false(self, msg: &str);
    }

    #[sealed::sealed]
    impl Expect for bool {
        #[inline]
        #[track_caller]
        fn assert_true(self) {
            assert!(self, "assertion failed: expected `true` but was `false`");
        }

        #[inline]
        #[track_caller]
        fn expect_true(self, msg: &str) {
            assert!(self, "{}", msg);
        }

        #[inline]
        #[track_caller]
        fn assert_false(self) {
            assert!(!self, "assertion failed: expected `false` but was `true`");
        }

        #[inline]
        #[track_caller]
        fn expect_false(self, msg: &str) {
            assert!(!self, "{}", msg);
        }
    }
}
