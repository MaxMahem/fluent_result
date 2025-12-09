macro_rules! test_no_panic {
    ($test_name:ident, $value:expr, $method:ident $(, $msg:expr)?) => {
        #[test]
        fn $test_name() {
            $value.$method($($msg),*);
        }
    };
}

macro_rules! test_panic {
    (dbg, $test_name:ident, $value:expr, $method:ident, $expected:expr $(, $msg:expr)?) => {
        #[test]
        #[cfg_attr(debug_assertions, should_panic(expected = $expected))]
        fn $test_name() {
            $value.$method($($msg),*);
        }
    };
    (rls, $test_name:ident, $value:expr, $method:ident, $expected:expr $(, $msg:expr)?) => {
        #[test]
        #[should_panic(expected = $expected)]
        fn $test_name() {
            $value.$method($($msg),*);
        }
    };
}

mod dbg_expect_none {
    use fluent_result::expect::dbg::ExpectNone;

    test_no_panic!(assert_none, None::<u8>, assert_none);
    test_no_panic!(expect_none, None::<u8>, expect_none, "test");

    test_panic!(dbg, assert_none_panic, Some(()), assert_none, "called `Option::assert_none()` on a `Some` value");
    test_panic!(dbg, expect_none_panic, Some(()), expect_none, "test", "test");
}

mod rls_expect_none {
    use fluent_result::expect::rls::ExpectNone;

    test_no_panic!(assert_none, None::<u8>, assert_none);
    test_no_panic!(expect_none, None::<u8>, expect_none, "test");

    test_panic!(rls, assert_none_panic, Some(()), assert_none, "called `Option::assert_none()` on a `Some` value");
    test_panic!(rls, expect_none_panic, Some(()), expect_none, "test", "test");
}
