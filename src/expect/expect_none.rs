/// Debug-only [`debug_assert!`] for [`Option`] values.
pub mod dbg {
    /// An extension trait for [`Option`] that allows [`debug_assert!`]ing the existance of the [`None`] variant.
    ///
    /// This trait only panics in debug mode (`cfg(debug_assertions)`). In release mode, it does
    /// nothing.
    #[sealed::sealed]
    pub trait ExpectNone {
        /// [`debug_assert!`] a [`Option`] is [`None`].
        ///
        /// This is useful for validating that a method that should return [`None`] does so. For
        /// example when inserting a value into a [`HashMap`](std::collections::HashMap) who's key
        /// should be unique.
        ///
        /// # Panics
        ///
        /// Panics in debug builds if the value is a [`Some`] variant.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use fluent_result::expect::rls::ExpectNone;
        /// use std::collections::HashMap;
        ///
        /// fn add_new_user(users: &mut HashMap<String, u32>, name: &str, id: u32) {
        ///     users.insert(name.to_string(), id).assert_none();
        /// }
        ///
        /// let mut users = HashMap::new();
        /// add_new_user(&mut users, "Alice", 1);
        /// ```
        fn assert_none(self);

        /// [`debug_assert!`]s a [`Option`] is [`None`] with `msg`.
        ///
        /// This is useful for validating that a method that should return [`None`] does so. For
        /// example when inserting a value into a [`HashMap`](std::collections::HashMap) who's key
        /// should be unique.
        ///
        /// # Panics
        ///
        /// Panics in debug builds with 'msg' if the value is a [`Some`] variant.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use fluent_result::expect::rls::ExpectNone;
        /// use std::collections::HashMap;
        ///
        /// fn add_new_user(users: &mut HashMap<String, u32>, name: &str, id: u32) {
        ///     users.insert(name.to_string(), id).expect_none("User already exists");
        /// }
        ///
        /// let mut users = HashMap::new();
        /// add_new_user(&mut users, "Alice", 1);
        /// ```
        fn expect_none(self, msg: &str);
    }

    #[sealed::sealed]
    impl<T> ExpectNone for Option<T> {
        #[inline]
        #[track_caller]
        fn assert_none(self) {
            debug_assert!(self.is_none(), "called `Option::assert_none()` on a `Some` value");
        }

        #[inline]
        #[track_caller]
        fn expect_none(self, msg: &str) {
            debug_assert!(self.is_none(), "{}", msg);
        }
    }
}

/// Release-mode [`assert!`] for [`Option`] values.
pub mod rls {
    /// An extension trait for [`Option<T>`] that allows [`assert!`]ing the existance of the [`None`] variant.
    #[sealed::sealed]
    pub trait ExpectNone {
        /// [`assert!`]s a [`Option`] is [`None`].
        ///
        /// This is useful for validating that a method that should return [`None`] does so. For
        /// example when inserting a value into a [`HashMap`](std::collections::HashMap) who's key
        /// should be unique.
        ///
        /// # Panics
        ///
        /// Panics if the value is a [`Some`] variant.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use fluent_result::expect::rls::ExpectNone;
        /// use std::collections::HashMap;
        ///
        /// fn add_new_user(users: &mut HashMap<String, u32>, name: &str, id: u32) {
        ///     users.insert(name.to_string(), id).assert_none();
        /// }
        ///
        /// let mut users = HashMap::new();
        /// add_new_user(&mut users, "Alice", 1);
        /// ```
        fn assert_none(self);

        /// [`assert!`]s a [`Option`] is [`None`] with `msg`.
        ///
        /// This is useful for validating that a method that should return [`None`] does so. For
        /// example when inserting a value into a [`HashMap`](std::collections::HashMap) who's key
        /// should be unique.
        ///
        /// # Panics
        ///
        /// Panics with 'msg' if the value is a [`Some`] variant.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use fluent_result::expect::rls::ExpectNone;
        /// use std::collections::HashMap;
        ///
        /// fn add_new_user(users: &mut HashMap<String, u32>, name: &str, id: u32) {
        ///     users.insert(name.to_string(), id).expect_none("User already exists");
        /// }
        ///
        /// let mut users = HashMap::new();
        /// add_new_user(&mut users, "Alice", 1);
        /// ```
        fn expect_none(self, msg: &str);
    }

    #[sealed::sealed]
    impl<T> ExpectNone for Option<T> {
        #[inline]
        #[track_caller]
        fn assert_none(self) {
            assert!(self.is_none(), "called `Option::assert_none()` on a `Some` value");
        }

        #[inline]
        #[track_caller]
        fn expect_none(self, msg: &str) {
            assert!(self.is_none(), "{}", msg);
        }
    }
}
